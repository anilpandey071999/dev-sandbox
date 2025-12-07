#[allow(warnings)]
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::future::Future;
use std::sync::LazyLock;
use std::time::Duration;

use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

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

async fn print_statement() {
    COUNTER.with(|counter| {
        let counter = unsafe { &mut *counter.get() };
        println!("counter: {:?}", counter);
    });
}

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(1)
        .on_thread_start(|| println!("Thread Starting for runtime A"))
        .on_thread_stop(|| println!("Thread Stopping for runtime A"))
        .thread_keep_alive(Duration::from_secs(60))
        .global_queue_interval(61)
        .on_thread_park(|| println!("Thread Parking for runtime A"))
        .thread_name("Our coustom runtime A")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_time()
        .build()
        .unwrap();
    runtime
});

pub fn spawn_task<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    RUNTIME.spawn(future)
}
#[allow(warnings)]
async fn sleep_example() -> i32 {
    println!("Sleeping for 2 Seconds");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("done sleeping!!");
    20
}

fn _coustom_tokio_main() {
    let handle = spawn_task(sleep_example());
    println!("spawned task");
    println!("task status: {}", handle.is_finished());
    std::thread::sleep(Duration::from_secs(3));
    println!("task status: {}", handle.is_finished());
    let result = RUNTIME.block_on(handle).unwrap();
    println!("task result: {}", result);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            println!("Hello, world!");
        });
    });
    let mut count = 0;
    loop {
        tokio::signal::ctrl_c().await.unwrap();
        println!("ctrl-c received!");
        count += 1;
        if count > 2 {
            std::process::exit(0);
        }
    }
    // tokio::spawn(cleanup());
    // let start = std::time::SystemTime::now();
    // println!("Start Time: {:#?}", start);
    // let pool = LocalPoolHandle::new(3);
    // let sequence = [1, 2, 3, 4, 5];
    // let repeated_sequence: Vec<_> = sequence.iter().cycle().take(5000).cloned().collect();
    // let mut futures = Vec::new();
    // for number in repeated_sequence {
    //     futures.push(pool.spawn_pinned(move || async move {
    //         something(number).await;
    //         something(number).await
    //     }));
    // }

    // for i in futures {
    //     let _ = i.await.unwrap();
    // }
    // let _ = pool
    //     .spawn_pinned(|| async { print_statement().await })
    //     .await
    //     .unwrap();
    // let end = std::time::SystemTime::now();
    // println!(
    //     "end Time: {:#?} And Dureation: {:#?}",
    //     end,
    //     end.duration_since(start).unwrap()
    // );
}

async fn cleanup() {
    println!("Cleanup background task started!!");
    let mut count = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        tokio::signal::ctrl_c().await.unwrap();
        println!("ctrl-c received!");
        count += 1;
        if count > 2 {
            std::process::exit(0);
        }
    }
}
