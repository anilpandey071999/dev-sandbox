use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI16};
use core::sync::atomic::Ordering;
use std::sync::LazyLock;
use std::future::Future;
use std::task::Poll;
use std::task::Context;
use std::time::{Instant, Duration};

static TEMP: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| {
   Arc::new(AtomicI16::new(2090)) 
});

static DESIRED_TEMP: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| {
    Arc::new(AtomicI16::new(2100))
});

static HEAT_ON: LazyLock<Arc<AtomicBool>> = LazyLock::new(||{
    Arc::new(AtomicBool::new(false))
});

pub struct DisplayFuture {
    pub temp_snapshort: i16
}

impl DisplayFuture {
    pub fn new() -> Self {
        DisplayFuture { temp_snapshort: TEMP.load(Ordering::SeqCst) }
    }
}

impl Future for DisplayFuture{
    type Output = ();
    
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_snapshort = TEMP.load(Ordering::SeqCst);
        let desired_temp = DESIRED_TEMP.load(Ordering::SeqCst);
        let heat_no = HEAT_ON.load(Ordering::SeqCst);
        
        if current_snapshort == self.temp_snapshort {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        if current_snapshort < desired_temp && heat_no{
            cx.waker().wake_by_ref();
            return Poll::Pending;
        } else if current_snapshort > desired_temp && heat_no == false {
            HEAT_ON.store(true, Ordering::SeqCst);
        }
        
        clearscreen::clear().unwrap();
        println!("Temperature: {}\nDesired Temp: {}\nHeater On: {}", 
            current_snapshort as f32 / 100.0, 
            desired_temp as f32 / 100.0, 
            heat_no
        );
        self.temp_snapshort = current_snapshort;
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI16, Ordering};
        
        let some_var = AtomicI16::new(5);
        
        assert_eq!(some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed), Ok(5));
        
        assert_eq!(some_var.load(Ordering::Relaxed), 10);
        
        assert_eq!(some_var.compare_exchange(5, 12, Ordering::SeqCst, Ordering::Acquire), Err(10));
        
        assert_eq!(some_var.load(Ordering::Relaxed), 10);
    }
}