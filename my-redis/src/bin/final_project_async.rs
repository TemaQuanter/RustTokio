use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
/// WARNING: This is a non-working code.
/// TODO: Solve issues with mutability.
use std::{
    task::{Context, Poll},
    time::Instant,
};
use tokio_stream::{Stream, StreamExt};

/// This structure is a modification of std::time::Delay that can be polled.
///
struct Delay {
    dead_line: Instant,
} // end struct Delay

impl Delay {
    /// This function creates an initialized instance of Delay.
    ///
    fn new(delay: Duration) -> Self {
        Self {
            dead_line: Instant::now() + delay,
        } // end Self
    } // end new()
} // end impl Delay

impl Future for Delay {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the time is over.
        if Instant::now() < self.dead_line {
            // The time is not over yet.

            // Create a copy of a waker and a delay.
            let waker = cx.waker().clone();
            let dead_line = self.dead_line;

            // Spawn a thread that will call the caller of 'await' after
            // enough time will have passed.
            tokio::spawn(async move {
                // Sleep until the future needs to be polled again.
                if dead_line > Instant::now() {
                    thread::sleep(dead_line - Instant::now());
                } // end if

                // Inform the caller that the future is ready to be called again.
                waker.wake();
            }); // end tokio::spawn()

            return Poll::Pending;
        } // end if

        Poll::Ready("Time's up!".to_string())
    } // end poll()
} // end impl Future for Delay.

/// This structure allows to return the elements of an iterative structure
/// in time intervals.
///
struct Interval<T>
where
    T: Iterator,
{
    delay: Delay,
    iter: Arc<Mutex<T>>,
    time_span: Duration,
} // end struct Interval

impl<T: Iterator> Stream for Interval<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Check if all the repetition have already been performed.
        if let Some(value) = Box::pin(&mut self.iter.lock().unwrap()).next() {
            match Pin::new(&mut self.delay).poll(cx) {
                Poll::Ready(_) => {
                    let delay = Delay::new(self.time_span);

                    self.delay = delay;

                    return Poll::Ready(None);
                }
                Poll::Pending => Poll::Pending,
            } // end match
        } else {
            return Poll::Ready(None);
        } // end if
    } // end poll_next()
} // end impl Stream for Interval

#[tokio::main]
async fn main() {
    let alarm: Delay = Delay::new(Duration::from_millis(1500));

    println!("{}", alarm.await);
} // end main()
