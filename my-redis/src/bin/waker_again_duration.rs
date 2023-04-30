use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use tokio::time::{Duration, Instant};

struct Alarm {
    dead_line: Instant
} // end struct Alarm

impl Future for Alarm {
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
} // end impl Future for Alarm

impl Alarm {
    fn new(dead_line: Duration) -> Self {
        Self {
            dead_line: Instant::now() + dead_line
        } // end Self
    } // end now()
} // end impl Alarm

#[tokio::main]
async fn main() {
    let alarm = Alarm::new(tokio::time::Duration::from_millis(2000));

    println!("{}", alarm.await);
} // end main()