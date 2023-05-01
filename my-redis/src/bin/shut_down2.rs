#[tokio::main]
async fn main() {
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(10);

    let cancellation_token: tokio_util::sync::CancellationToken =
        tokio_util::sync::CancellationToken::new();

    for i in 0..10 {
        tasks.push(tokio::spawn(rand_task(i, cancellation_token.clone())));
    } // end for

    // Await all the tasks to complete.
    for tsk in tasks.into_iter() {
        tokio::select! {
            _ = tsk => {}
            _ = tokio::signal::ctrl_c() => {
                println!("The program is cancelled");
                println!("Stopping all the processes");
                cancellation_token.cancel();
            }
        } // end select
    } // end for

    println!("The program is finished!");
} // end main()

async fn rand_task(id: usize, cancellation_token: tokio_util::sync::CancellationToken) {
    tokio::select! {
        _ = async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10000 - 500 * (id as u64))).await;
            println!("Task #{id} completed!");
        } => {},
        _ = cancellation_token.cancelled() => {
            println!("Task #{id} was cancelled");
        }
    } // end select
} // end rand_task()
