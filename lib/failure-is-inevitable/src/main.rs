use std::collections::HashMap;
use std::future::Future;

use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, sleep, timeout},
};

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

async fn add_delay_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

async fn worker(mut rx: mpsc::Receiver<Command>) {
    // println!("working");
    let mut global_state: HashMap<String, String> = HashMap::new();
    while let Some(command_ops) = rx.recv().await {
        match command_ops {
            Command::Set { key, value, resp } => {
                let _ = global_state.insert(key, value);
                let delay_ms = rand::random_range(50..100);
                add_delay_ms(delay_ms).await;
                let _ = resp.send(Ok(()));
            }
            Command::Get { key, resp } => {
                let delay_ms = rand::random_range(70..100);
                add_delay_ms(delay_ms).await;
                if let Some(res) = global_state.get(&key).cloned() {
                    let _ = resp.send(Ok(Some(res)));
                } else {
                    let _ = resp.send(Ok(None));
                }
            }
            Command::Delete { key, resp } => {
                let exist = global_state.remove(&key).is_some();
                let delay_ms = rand::random_range(50..100);
                add_delay_ms(delay_ms).await;
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
    for i in 0..10 {
        let local_tx = tx.clone();
        let handler = tokio::spawn(async move {
            let result = retry_with_timeout(|| {
                let tx = local_tx.clone();
                async move {
                    let (resp_tx, resp_rx) = oneshot::channel();

                    let _ = tx
                        .send(Command::Set {
                            key: "anil".to_string(),
                            value: format!("pandey-{i}"),
                            resp: resp_tx,
                        })
                        .await;

                    resp_rx
                }
            })
            .await;

            println!("task {i} -> {:?}", result);
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

async fn retry_with_timeout<F, Fut, T>(mut op: F) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = oneshot::Receiver<Result<T, String>>>,
{
    let mut backoff = 100;

    for attempt in 1..=3 {
        let rx = op().await;

        match timeout(Duration::from_millis(85), rx).await {
            Ok(Ok(Ok(val))) => return Ok(val),

            Ok(Ok(Err(e))) => {
                if attempt == 3 {
                    return Err(e);
                }
            }

            Ok(Err(_)) => {
                if attempt == 3 {
                    return Err("response channel dropped".into());
                }
            }

            Err(_) => {
                if attempt == 3 {
                    return Err("timeout".into());
                }
            }
        }

        sleep(Duration::from_millis(backoff)).await;
        backoff *= 2;
    }

    unreachable!()
}
