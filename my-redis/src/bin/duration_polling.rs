use std::{
    future::Future,
    time::Duration,
    task::{
        Context,
        Poll
    },
    pin::Pin
};

struct Alarm {
    ftr: Pin<Box<dyn Future<Output = String>>>
} // end struct Alarm

impl Future for Alarm {
    type Output = String;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = self.ftr.as_mut().poll(cx) {
            Poll::Ready(val)
        } else {
            Poll::Pending
        } // end if
    } // end poll()
} // end impl Future for Alarm

async fn sleep(duration: Duration) -> String {
    tokio::time::sleep(duration).await;
    "Time's up!".to_string()
} // end sleep()

#[tokio::main]
async fn main() {
    let alarm: Alarm = Alarm { ftr: Box::pin(sleep(Duration::from_millis(1500)))};

    println!("{}", alarm.await);
} // end main()