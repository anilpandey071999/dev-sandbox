use std::future::Future;
use std::{collections::HashMap, sync::OnceLock};

use opentelemetry::global::{self, BoxedTracer};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout::SpanExporter;
use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, sleep, timeout},
};
use tracing::{Dispatch, Instrument, info, info_span, warn};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        let span = tracing::info_span!("worker_command");
        let _enter = span.enter();

        match command_ops {
            Command::Set { key, value, resp } => {
                info!(key = %key, "set");
                let _ = global_state.insert(key, value);
                let delay_ms = rand::random_range(50..100);
                add_delay_ms(delay_ms).await;
                let _ = resp.send(Ok(()));
            }
            Command::Get { key, resp } => {
                let delay_ms = rand::random_range(70..100);
                add_delay_ms(delay_ms).await;
                info!(key = %key, "get");
                if let Some(res) = global_state.get(&key).cloned() {
                    let _ = resp.send(Ok(Some(res)));
                } else {
                    let _ = resp.send(Ok(None));
                }
            }
            Command::Delete { key, resp } => {
                info!(key = %key, "delete");
                let exist = global_state.remove(&key).is_some();
                let delay_ms = rand::random_range(50..100);
                add_delay_ms(delay_ms).await;
                let _ = resp.send(Ok(exist));
            }
            Command::Shutdown => {
                info!("shutdown");
                break;
            }
        }
    }
}

fn get_tracer() -> &'static BoxedTracer {
    static TRACER: OnceLock<BoxedTracer> = OnceLock::new();
    TRACER.get_or_init(|| global::tracer("KeyValue Server"))
}

fn init_tracer_provider() -> (SdkTracerProvider, tracing::dispatcher::DefaultGuard) {
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();

    let tracer = provider.tracer("kv-server");
    let otel_layer = OpenTelemetryLayer::new(tracer);

    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer);

    let guard = tracing::dispatcher::set_default(&Dispatch::new(subscriber));

    (provider, guard)
}

#[tokio::main]
async fn main() {
    let (tracer_provider, _guard) = init_tracer_provider();

    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(worker(rx));

    let mut handlers = Vec::new();
    for i in 0..10 {
        let local_tx = tx.clone();
        let span = info_span!("client_task", task_id = i);
        let handler = tokio::spawn(
            async move {
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
    sleep(Duration::from_millis(100)).await;
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

    for attempt in 1..=3 {
        let rx = async { op().await }
            .instrument(info_span!("retry_attempt", attempt))
            .await;

        match timeout(Duration::from_millis(85), rx).await {
            Ok(Ok(Ok(val))) => {
                info!("success");
                return Ok(val);
            }

            Ok(Ok(Err(e))) => {
                warn!("failed attempt");
                if attempt == 3 {
                    return Err(e);
                }
            }

            Ok(Err(_)) => {
                warn!("channel dropped");
                if attempt == 3 {
                    return Err("response channel dropped".into());
                }
            }

            Err(_) => {
                if attempt == 3 {
                    warn!("timeout");
                    return Err("timeout".into());
                }
            }
        }

        sleep(Duration::from_millis(backoff)).await;
        backoff *= 2;
    }

    unreachable!()
}
