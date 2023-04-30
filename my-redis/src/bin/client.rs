use mini_redis::client::{self, Client};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;

use bytes::Bytes;

type Responder<T> = tokio::sync::oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
} // end enum Command

#[tokio::main]
async fn main() {
    // Create a new channel to facilitate the communication between tasks.
    let (tx, mut rx): (Sender<Command>, Receiver<Command>) = mpsc::channel(32);

    // Establish a connection with a server.
    let manager: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let mut client: Client = client::connect("127.0.0.1:6666")
            .await
            .expect("Failed to connect to the server");

        // Receive the messages from the transmitters.
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;

                    // Ignore errors.
                    let _ = resp.send(res);
                } // end Get
                Set { key, value, resp } => {
                    let res = client.set(&key, value).await;

                    // Ignore errors.
                    let _ = resp.send(res);
                } // end Set
            } // end match
        } // end while
    });

    // Clone tx to put it in different tasks.
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // Create 2 tasks that are sending messages.
    let task1 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let cmd: Command = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: oneshot_tx,
        };

        // Send the request.
        tx1.send(cmd).await.expect("Failed to send a message");

        // Await the response.
        let res = oneshot_rx.await;

        println!("I am task1");
        println!("GOT = {:?}", res);
    }); // end task1

    task1.await.expect("An error occurred in the first task");

    let task2 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();
        let cmd: Command = Command::Get {
            key: "foo".to_string(),
            resp: oneshot_tx,
        };

        // Send the request.
        tx2.send(cmd).await.expect("Failed to send a message");

        // Await the response.
        let res = oneshot_rx.await;

        println!("I am task2");
        println!("GOT = {:?}", res);
    }); // end task2

    task2.await.expect("An error occurred in the second task");

    manager
        .await
        .expect("An error occurred in the task manager");
} // end main()
