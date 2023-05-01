#[tokio::main]
async fn main() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("I haven't finished yet :(");
        }
        _ = some_random_task() => {}
    }
} // end main()

async fn some_random_task() {
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    println!("I am done!");
} // end some_random_task()
