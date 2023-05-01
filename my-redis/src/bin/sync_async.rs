use std::thread;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use tokio::time::Duration;

fn main() {
    let rt: Runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .unwrap();

    let mut vc: Vec<JoinHandle<()>> = Vec::with_capacity(15);

    for i in 0..15 {
        vc.push(rt.spawn(func(i)));
    } // end for

    thread::sleep(Duration::from_millis(730));

    println!("Hi from the main thread!");

    // Await all asynchronous tasks.
    for tsk in vc.into_iter() {
        rt.block_on(tsk).expect("The task has failed");
    } // end for
} // end main()

async fn func(id: usize) {
    tokio::time::sleep(Duration::from_millis(1000 - 50 * (id as u64))).await;
    println!("Task #{id} is done!");
} // end func()
