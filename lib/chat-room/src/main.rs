use std::{env::args, str::FromStr, sync::Arc};

use dashmap::DashMap;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

pub enum Role {
    Server,
    Client,
}

impl FromStr for Role {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" | "Server" | "SERVER" => Ok(Role::Server),
            "client" | "Client" | "CLIENT" => Ok(Role::Client),
            _ => Err("pass the valid argument"),
        }
    }
}

/* ========================= SERVER ========================= */

async fn build_server() {
    println!("[SERVER] starting on 127.0.0.1:2345");

    let listener = TcpListener::bind("127.0.0.1:2345")
        .await
        .expect("Failed to start tcp listener");

    let (router_tx, mut router_rx) = mpsc::channel::<(usize, String)>(512);
    let clients: Arc<DashMap<usize, mpsc::Sender<String>>> = Arc::new(DashMap::new());

    /* ---------- ROUTER TASK ---------- */
    let clients_for_router = Arc::clone(&clients);
    tokio::spawn(async move {
        println!("[ROUTER] started");

        while let Some((from, msg)) = router_rx.recv().await {
            println!(
                "[ROUTER] received message from client {} → broadcasting",
                from
            );

            let targets: Vec<_> = clients_for_router
                .iter()
                .filter(|e| *e.key() != from)
                .map(|e| e.value().clone())
                .collect();

            for tx in targets {
                let _ = tx.send(msg.clone()).await;
            }
        }

        println!("[ROUTER] stopped");
    });

    let mut next_client_id: usize = 0;

    /* ---------- ACCEPT LOOP ---------- */
    loop {
        let (tcp_stream, addr) = listener.accept().await.unwrap();

        let client_id = next_client_id;
        next_client_id += 1;

        println!(
            "[SERVER] new client connected: id={} addr={}",
            client_id, addr
        );

        let (personal_tx, personal_rx) = mpsc::channel::<String>(512);
        clients.insert(client_id, personal_tx);

        println!(
            "[SERVER] registered client {} (total={})",
            client_id,
            clients.len()
        );

        let router_tx = router_tx.clone();
        let clients_clone = Arc::clone(&clients);

        tokio::spawn(async move {
            let _ =
                handle_client(tcp_stream, client_id, router_tx, personal_rx, clients_clone).await;
        });
    }
}

async fn handle_client(
    tcp_stream: TcpStream,
    client_id: usize,
    router_tx: mpsc::Sender<(usize, String)>,
    mut personal_rx: mpsc::Receiver<String>,
    clients: Arc<DashMap<usize, mpsc::Sender<String>>>,
) -> io::Result<()> {
    println!("[CLIENT {}] handler started", client_id);

    let (mut reader, mut writer) = tcp_stream.into_split();

    /* ---------- READ TASK ---------- */
    let read_task = tokio::spawn({
        let router_tx = router_tx.clone();
        async move {
            let mut buf = [0u8; 512];

            loop {
                let n = reader.read(&mut buf).await?;
                if n == 0 {
                    println!("[CLIENT {}] socket closed", client_id);
                    break;
                }

                println!("[CLIENT {}] read {} bytes → router", client_id, n);

                let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                let _ = router_tx.send((client_id, msg)).await;
            }

            Ok::<(), io::Error>(())
        }
    });

    /* ---------- WRITE LOOP ---------- */
    while let Some(msg) = personal_rx.recv().await {
        println!("[CLIENT {}] sending message to socket", client_id);
        writer.write_all(msg.as_bytes()).await?;
    }

    println!("[CLIENT {}] disconnecting", client_id);
    clients.remove(&client_id);

    println!(
        "[CLIENT {}] removed (remaining={})",
        client_id,
        clients.len()
    );

    read_task.await??;
    Ok(())
}

/* ========================= CLIENT ========================= */

async fn build_client() -> io::Result<()> {
    println!("[CLIENT] connecting to server...");
    let stream = TcpStream::connect("127.0.0.1:2345").await?;
    println!("[CLIENT] connected");

    let (mut reader, mut writer) = stream.into_split();

    /* ---------- STDIN → SERVER ---------- */
    let write_task = tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 512];

        loop {
            let n = stdin.read(&mut buf).await?;
            if n == 0 {
                break;
            }

            println!("[CLIENT] sending {} bytes", n);
            writer.write_all(&buf[..n]).await?;
        }

        Ok::<(), io::Error>(())
    });

    /* ---------- SERVER → STDOUT ---------- */
    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 512];

        loop {
            let n = reader.read(&mut buf).await?;
            if n == 0 {
                println!("[CLIENT] server closed connection");
                break;
            }

            println!("[CLIENT] received: {}", String::from_utf8_lossy(&buf[..n]));
        }

        Ok::<(), io::Error>(())
    });

    tokio::try_join!(write_task, read_task)?;
    Ok(())
}

/* ========================= MAIN ========================= */

#[tokio::main]
async fn main() {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("argument is missing: server | client");
    }

    match args[1].parse::<Role>() {
        Ok(Role::Server) => build_server().await,
        Ok(Role::Client) => build_client().await.unwrap(),
        Err(err) => panic!("{err}"),
    }
}
