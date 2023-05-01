use tokio::sync::oneshot;

async fn send_tx(tx: oneshot::Sender<i32>, num: i32) {
    tx.send(num).expect("Failed to send a message through the channel");
} // end send_tx()

#[tokio::main]
async fn main() {
    let (tx1, rx1) : (oneshot::Sender<i32>, oneshot::Receiver<i32>) = oneshot::channel();
    let (tx2, rx2) : (oneshot::Sender<i32>, oneshot::Receiver<i32>) = oneshot::channel();

    tokio::spawn(send_tx(tx1, 13));
    tokio::spawn(send_tx(tx2, 24));

    let res = tokio::select! {
        Ok(res1) = rx1 => { res1 },
        Ok(res2) = rx2 => { res2 }
    }; // end tokio::select!()

    println!("{}", res);
} // end main()