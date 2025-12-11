use std::os::macos::raw::stat;

use tokio::sync::{
    mpsc::channel,
    mpsc::{Receiver, Sender},
    oneshot,
};

struct Message {
    value: i64,
    responder: oneshot::Sender<i64>,
}

async fn basic_actor(mut rx: Receiver<Message>) {
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
