use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut vc: tokio_stream::Iter<std::slice::Iter<i32>> = tokio_stream::iter(&[5, 3, -2, 12]);

    while let Some(val) = vc.next().await {
        println!("{}", val);
    } // end while
} // end main()
