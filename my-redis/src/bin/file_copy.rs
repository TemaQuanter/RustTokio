use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open a file in a writing mode.
    let mut file = File::create("bar.txt").await?;

    // Create a buffer with some data to write in a file.
    let mut buff: &[u8] = b"Something good is about to happen";

    io::copy(&mut buff, &mut file).await?;
    Ok(())
} // end main()
