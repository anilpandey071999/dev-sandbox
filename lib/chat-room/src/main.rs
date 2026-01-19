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

async fn build_server() {
    let listener = TcpListener::bind("127.0.0.1:2345")
        .await
        .expect("Failed to start tcp listener");

    let (router_tx, mut router_rx) = mpsc::channel::<(usize, String)>(512);

    let mut clients: Arc<DashMap<usize, mpsc::Sender<String>>> = Arc::new(DashMap::new());

    let clients_for_task = Arc::clone(&clients);

    tokio::spawn(async move {
        while let Some((from, msg)) = router_rx.recv().await {
            let senders: Vec<_> = clients_for_task
                .iter()
                .filter(|e| *e.key() != from)
                .map(|e| e.value().clone())
                .collect();
            for sender in senders {
                let _ = sender.send(msg.clone()).await;
            }
        }
    });

    let mut next_client_id: usize = 0;

    loop {
        let (tcp_strem, _) = listener.accept().await.unwrap();

        let client_id = next_client_id;
        next_client_id += 1;

        let (personal_tx, personal_rx) = mpsc::channel::<String>(512);
        clients.insert(client_id, personal_tx.clone());
        let clients_clone = Arc::clone(&clients);

        tokio::spawn(handle_client(
            tcp_strem,
            client_id,
            router_tx.clone(),
            personal_rx,
            clients_clone,
        ));
    }
}

async fn handle_client(
    tcp_strem: TcpStream,
    client_id: usize,
    router_tx: mpsc::Sender<(usize, String)>,
    mut personal_rx: mpsc::Receiver<String>,
    clients: Arc<DashMap<usize, mpsc::Sender<String>>>,
) -> io::Result<()> {
    let (mut reader, mut writer) = tcp_strem.into_split();
    let read_task = tokio::spawn({
        let router_tx = router_tx.clone();

        async move {
            let mut buf = [0u8; 512];

            loop {
                let n = reader.read(&mut buf).await?;
                if n == 0 {
                    break;
                }

                let message = String::from_utf8_lossy(&buf[..n]).to_string();

                let _ = router_tx.send((client_id, message)).await;
            }
            Ok::<(), io::Error>(())
        }
    });

    while let Some(msg) = personal_rx.recv().await {
        writer.write_all(msg.as_bytes()).await?;
    }

    clients.remove(&client_id);

    read_task.await??;
    Ok::<(), io::Error>(())
}

async fn build_client() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:2345").await?;
    let (mut reader, mut writer) = stream.into_split();

    let write_task = tokio::spawn(async move {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 512];

        loop {
            let n = stdin.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            writer.write_all(&buf[..n]).await?;
        }

        Ok::<(), io::Error>(())
    });

    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 512];

        loop {
            let n = reader.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            println!("received: {}", String::from_utf8_lossy(&buf[..n]));
        }
        Ok::<(), io::Error>(())
    });
    tokio::try_join!(write_task, read_task)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("argument is missing")
    }

    match args[1].parse::<Role>() {
        Ok(arg) => match arg {
            Role::Server => build_server().await,
            Role::Client => build_client().await.unwrap(),
        },
        Err(err) => panic!("{err}"),
    }

    // println!("Hello, world! {:?}", arg);
}
