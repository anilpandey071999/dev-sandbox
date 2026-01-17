use std::future::Future;
use std::{collections::HashMap, time::Instant};

use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, sleep, timeout},
};
use tracing::{Instrument, error, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
                async {
                    info!(key = %key, "set");
                    let _ = global_state.insert(key, value);
                    let delay_ms = rand::random_range(5..50);
                    add_delay_ms(delay_ms).await;
                    let _ = resp.send(Ok(()));
                }
                .instrument(info_span!("worker_command", command = "set"))
                .await;
            }
            Command::Get { key, resp } => {
                async {
                    let delay_ms = rand::random_range(70..100);
                    add_delay_ms(delay_ms).await;
                    info!(key = %key, "get");
                    if let Some(res) = global_state.get(&key).cloned() {
                        let _ = resp.send(Ok(Some(res)));
                    } else {
                        let _ = resp.send(Ok(None));
                    }
                }
                .instrument(info_span!("worker_command", command = "get"))
                .await;
            }
            Command::Delete { key, resp } => {
                async {
                    info!(key = %key, "delete");
                    let exist = global_state.remove(&key).is_some();
                    let delay_ms = rand::random_range(50..100);
                    add_delay_ms(delay_ms).await;
                    let _ = resp.send(Ok(exist));
                }
                .instrument(info_span!("worker_command", command = "delete"))
                .await;
            }
            Command::Shutdown => {
                info!("shutdown");
                break;
            }
        }
    }
}

fn init_tracer_provider() -> SdkTracerProvider {
    let provider = SdkTracerProvider::builder().build();

    let tracer = provider.tracer("kv-server");

    let log_layer = tracing_subscriber::fmt::layer()
        .with_level(true) // INFO / WARN / ERROR
        .with_target(false) // hide crate name
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()) // ✅ RUST_LOG
        .with(log_layer)
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    provider
}

fn main() {
    let tracer_provider = init_tracer_provider();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(worker(rx));

        let mut handlers = Vec::new();
        for i in 0..10 {
            let local_tx = tx.clone();
            let span = info_span!("client_task", task_id = i);
            let handler = tokio::spawn(
                async move {
                    let _result = retry_with_timeout(|| {
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

                    // println!("task {i} -> {:?}", result);
                }
                .instrument(span),
            );
            handlers.push(handler);
        }
        for handler in handlers {
            handler
                .await
                .map_err(|err| println!("Failed here {err}"))
                .unwrap();
        }
        tx.send(Command::Shutdown).await.unwrap();
    });
    // sleep(Duration::from_millis(100)).await;
    if let Err(err) = tracer_provider.shutdown() {
        panic!("Failed to shortdown!! {err}")
    }
}

async fn retry_with_timeout<F, Fut, T>(mut op: F) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = oneshot::Receiver<Result<T, String>>>,
{
    let mut backoff = 100;
    let start = Instant::now();

    for attempt in 1..=3 {
        let rx = async { op().await }
            .instrument(info_span!("retry_attempt", attempt))
            .await;

        match timeout(Duration::from_millis(85), rx).await {
            Ok(Ok(Ok(val))) => {
                let elapsed = start.elapsed().as_millis();
                info!(attempt, elapsed_ms = elapsed, "success");
                return Ok(val);
            }

            Ok(Ok(Err(e))) => {
                error!(
                    attempt,
                    error = %e,
                    "operation failed"
                );
                return Err(e);
            }

            Ok(Err(_)) => {
                error!(attempt, "response channel dropped");
                return Err("response channel dropped".into());
            }

            Err(_) => {
                let elapsed = start.elapsed().as_millis();

                if attempt < 3 {
                    warn!(attempt, elapsed_ms = elapsed, "timeout");
                } else {
                    error!(attempt, elapsed_ms = elapsed, "timeout");
                    return Err("timeout".into());
                }
            }
        }

        sleep(Duration::from_millis(backoff)).await;
        backoff *= 2;
    }

    unreachable!()
}
