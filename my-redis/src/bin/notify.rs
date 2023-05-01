use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    delay(Duration::from_millis(5000)).await;
    println!("Hello, world after 5 seconds of waiting:)))");
}

async fn delay(delay: Duration) {
    let when = Instant::now() + delay;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });

    notify.notified().await;
}
