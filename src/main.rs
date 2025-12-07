#[allow(warnings)]
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::LazyLock;

use tokio::runtime::{Builder, Runtime};
use tokio::signal::unix::{Signal, SignalKind};

#[allow(warnings)]
use tokio::time;
use tokio_util::task::LocalPoolHandle;

#[allow(warnings)]
/// ### High Priority task will served!!
static HIGH_PRIORITY: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("High Priority Runtime")
        .enable_time()
        .build()
        .unwrap()
});
#[allow(warnings)]
/// Low Priority task will served!!
static LOW_PRIORITY: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("Low Priority Runtime")
        .enable_time()
        .build()
        .unwrap()
});

thread_local! {
    pub static COUNTER: UnsafeCell<HashMap<u32, u32>> = UnsafeCell::new(HashMap::new());
}

async fn something(number: u32) -> u32 {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    COUNTER.with(|counter| {
        let counter = unsafe { &mut *counter.get() };
        match counter.get_mut(&number) {
            Some(count) => {
                let placeholder = *count + 1;
                *count = placeholder;
            }
            None => {
                counter.insert(number, 1);
            }
        }
        // *counter.borrow_mut() += 1;
        // println!("Counter: {} for: {}", *counter.borrow(), number)
    });
    number
}

async fn _print_statement() {
    COUNTER.with(|counter| {
        let counter = unsafe { &mut *counter.get() };
        println!("counter: {:?}", counter);
    });
}

static RUNTIME: LazyLock<LocalPoolHandle> = LazyLock::new(|| LocalPoolHandle::new(4));

fn extract_data_from_thread() -> HashMap<u32, u32> {
    let mut extract_count = HashMap::new();
    COUNTER.with(|count| {
        let count = unsafe { &mut *count.get() };
        extract_count = count.clone();
    });
    return extract_count;
}

async fn get_complete_count() -> HashMap<u32, u32> {
    let mut complete_count = HashMap::new();
    let mut extract_count = Vec::new();
    for i in 0..4 {
        extract_count
            .push(RUNTIME.spawn_pinned_by_idx(|| async move { extract_data_from_thread() }, i));
    }
    for counter_future in extract_count {
        let extract_counter = counter_future.await.unwrap_or_default();
        for (key, count) in extract_counter {
            *complete_count.entry(key).or_insert(0) += count;
        }
    }
    return complete_count;
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _handle = tokio::spawn(async {
        let sequence = [1, 2, 3, 4, 5];
        let repeated_sequence: Vec<_> = sequence.iter().cycle().take(500000).cloned().collect();
        let mut futures = Vec::new();
        for number in repeated_sequence {
            futures.push(RUNTIME.spawn_pinned(move || async move {
                something(number).await;
                something(number).await
            }));
        }
        for i in futures {
            let _ = i.await.unwrap();
        }
        println!("All futures completed");
    });
    tokio::signal::ctrl_c().await.unwrap();
    println!("ctrl-c received!");
    let complete_counter = get_complete_count().await;
    println!("Complete counter: {:?}", complete_counter);
}
