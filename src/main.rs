use std::sync::Arc;
use tokio::sync::{
    Mutex,
    mpsc::channel,
    mpsc::Receiver,
    oneshot,
};

struct Message {
    value: i64,
    responder: oneshot::Sender<i64>,
}

async fn _basic_actor(mut rx: Receiver<Message>) {
    let mut state = 0;

    while let Some(msg) = rx.recv().await {
        state += msg.value;
        println!("Received: {}", msg.value);
        println!("State: {}", state);
    }
}

async fn resp_actor(mut rx: Receiver<Message>) {
    let mut state = 0;

    while let Some(msg) = rx.recv().await {
        state += msg.value;
        if msg.responder.send(state).is_err() {
            eprintln!("Failed to send response");
        }
    }
}

async fn _actor_replacement(state: Arc<Mutex<i64>>, value: i64) -> i64 {
    let mut state = state.lock().await;
    *state += value;
    return *state;
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Message>(100);

    // let _actor_handle = tokio::spawn(basic_actor(rx));
    let _resp_actor_handle = tokio::spawn(resp_actor(rx));

    for i in 1..10 {
        let (resp_tx, resp_rx) = oneshot::channel();
        let msg = Message {
            value: i,
            responder: resp_tx,
        };
        tx.send(msg).await.unwrap();
        println!("Respone: {}", resp_rx.await.unwrap());
    }
}

#[cfg(test)]
mod actor_model_tests {
    use super::*;

    #[tokio::test]
    async fn test_actor_replacement() {
        let state = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        let now = tokio::time::Instant::now();
        // 0..1_000_000_00
        for i in  0..100_000_000{
            let state_ref = state.clone();
            let future = async move {
                let handle = tokio::spawn(async move { _actor_replacement(state_ref, i).await });
                let _ = handle.await.unwrap();
            };
            handles.push(tokio::spawn(future));
        }
        for handle in handles {
            let _ = handle.await.unwrap();
        }

        println!("Time elapsed: {:?}", now.elapsed());
    }
}
