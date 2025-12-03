use std::cell::RefCell;
use std::future::Future;
use std::sync::LazyLock;
use std::time::Duration;

use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

use tokio_util::task::LocalPoolHandle;

/// ### High Priority task will served!!
static HIGH_PRIORITY: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("High Priority Runtime")
        .enable_time()
        .build()
        .unwrap()
});

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
    pub static COUNTER: RefCell<u32> = RefCell::new(1);
}

async fn something(number: u32) -> u32 {
    std::thread::sleep(std::time::Duration::from_secs(3));
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        println!("Counter: {} for: {}", *counter.borrow(), number)
    });
    number
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
    let pool = LocalPoolHandle::new(3);
    let one = pool.spawn_pinned_by_idx(|| async {
        println!("one");
        something(1).await
    }, 0);
    let two = pool.spawn_pinned_by_idx(|| async {
        println!("two");
        something(2).await
    }, 0);
    let three = pool.spawn_pinned_by_idx(|| async {
        println!("three");
        something(3).await
    }, 0);

    let result = async {
        let one = one.await.unwrap();
        let two = two.await.unwrap();
        let three = three.await.unwrap();
        one + two + three
    };
    println!("result: {}", result.await);
}
