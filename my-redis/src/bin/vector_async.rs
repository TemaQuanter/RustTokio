use rand::rngs::OsRng;
use rand::Rng;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    // Declare an array that is going to store futures.
    let mut vc: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    // Declare a random generator that could be shared between futures.
    let rand_rng = Arc::new(Mutex::new(OsRng::default()));

    // Add several spawned futures in the vector.
    for i in 0..30 {
        let rand_rng = Arc::clone(&rand_rng);
        vc.push(tokio::spawn(say_something(i, rand_rng))); // end push
    } // end for

    // Await all the futures.
    for ftr in vc.into_iter() {
        ftr.await.expect("Failed to join a task");
    } // end for
}

async fn say_something(num: usize, rand_rng: Arc<Mutex<OsRng>>) {
    let timer: u64;
    {
        // Randomly decide what amount of time the task will sleep.
        let mut rand_rng = rand_rng.lock().unwrap();
        timer = rand_rng.gen_range(0..300);
    }
    // Sleep for a random amount of time.
    tokio::time::sleep(tokio::time::Duration::from_millis(timer)).await;
    println!("Hi from task #{}!", num);
}
