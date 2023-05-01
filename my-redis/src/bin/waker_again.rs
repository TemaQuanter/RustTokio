use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use tokio::time::Instant;

struct Greeter {
    dead_line: Instant,
} // end struct Greeter

impl Future for Greeter {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the deadline has already been reached.
        if self.dead_line <= Instant::now() {
            Poll::Ready("Time is up!")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        } // end else
    } // end poll
} // end impl Future for Greeter

impl Greeter {
    fn new(dead_line: Instant) -> Self {
        Self { dead_line } // end Self
    } // end now()
} // end impl Greeter

#[tokio::main]
async fn main() {
    let ftr = Greeter::new(Instant::now() + tokio::time::Duration::from_millis(2000));

    println!("{}", ftr.await);
} // end main()
