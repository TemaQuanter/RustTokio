use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use tokio::time::Instant;

struct Greeter {
    dead_line: Instant
} // end struct Greeter

impl Future for Greeter {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the deadline has already been reached.
        if self.dead_line <= Instant::now() {
            Poll::Ready("Time is up!")
        } else {
            let cloned_waker = cx.waker().clone();
            let dead_line = self.dead_line;

            // Generate a new thread that will inform a caller that
            // the task is ready to be called again.
            thread::spawn(move || {
                if dead_line < Instant::now() {
                    thread::sleep(dead_line - Instant::now());
                } // end if
                cloned_waker.wake();
            });

            Poll::Pending
        } // end else
    } // end poll
} // end impl Future for Greeter

impl Greeter {
    fn new(dead_line: Instant) -> Self {
        Self {
            dead_line
        } // end Self
    } // end now()
} // end impl Greeter

#[tokio::main]
async fn main() {
    let ftr = Greeter::new(Instant::now() + tokio::time::Duration::from_millis(2000));

    println!("{}", ftr.await);
} // end main()