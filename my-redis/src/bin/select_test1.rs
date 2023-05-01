use std::future::{Future, Pending, Ready};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::sync::oneshot;

struct Race {
    racer1: oneshot::Receiver<&'static str>,
    racer2: oneshot::Receiver<&'static str>,
} // end struct Race

impl Future for Race {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.racer1).poll(cx) {
            println!("The first racer won and returned the value {:?}!", val);
            return Poll::Ready(());
        } // end if let

        if let Poll::Ready(val) = Pin::new(&mut self.racer2).poll(cx) {
            println!("The second racer won and returned the value {:?}", val);
            return Poll::Ready(());
        } // end if let

        Poll::Pending
    } // end fn poll()
} // end impl Future for Race

#[tokio::main]
async fn main() {
    let (tx1, rx1): (oneshot::Sender<&str>, oneshot::Receiver<&str>) = oneshot::channel();
    let (tx2, rx2): (oneshot::Sender<&str>, oneshot::Receiver<&str>) = oneshot::channel();

    let ftr: Race = Race {
        racer1: rx1,
        racer2: rx2,
    };

    tx1.send("Foo")
        .expect("Failed to send the data through the channel");
    tx2.send("Bar")
        .expect("Failed to send the data through the channel");

    ftr.await;
} // end main()
