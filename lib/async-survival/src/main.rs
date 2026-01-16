use tokio::sync::{mpsc, oneshot};

pub enum Command {
    Set {
        key: String,
        value: String,
        resp: oneshot::Sender<Result<(), String>>,
    },
    Get {
        key: String,
        resp: oneshot::Sender<Result<Option<String>, String>>,
    },
    Delete {
        key: String,
        resp: oneshot::Sender<Result<bool, String>>,
    },
    Shutdown,
}

async fn worker(mut rx: mpsc::Receiver<Command>) {
    // println!("working");
    let mut global_state: HashMap<String, String> = HashMap::new();
    while let Some(command_ops) = rx.recv().await {
        match command_ops {
            Command::Set { key, value, resp } => {
                let _ = global_state.insert(key, value);
                let _ = resp.send(Ok(()));
            }
            Command::Get { key, resp } => {
                if let Some(res) = global_state.get(&key).cloned() {
                    let _ = resp.send(Ok(Some(res)));
                } else {
                    let _ = resp.send(Ok(None));
                }
            }
            Command::Delete { key, resp } => {
                let exist = global_state.remove(&key).is_some();
                let _ = resp.send(Ok(exist));
            }
            Command::Shutdown => break,
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(worker(rx));

    let mut handlers = Vec::new();
    for i in 0..100 {
        let local_tx = tx.clone();
        let handler = tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            local_tx
                .send(Command::Set {
                    key: "anil".to_string(),
                    value: "pandey".to_string(),
                    resp: resp_tx,
                })
                .await
                .map_err(|err| println!("GOT error {}", err))
                .unwrap();

            println!("therad {i} | Set result: {:?}", resp_rx.await.unwrap());
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler
            .await
            .map_err(|err| println!("Failed here {err}"))
            .unwrap();
    }
    tx.send(Command::Shutdown).await.unwrap();
}
