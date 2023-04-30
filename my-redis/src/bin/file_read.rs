use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::open("foo.txt").await?;
    let mut buffer: Vec<u8> = Vec::new();

    // Read up to 10 bytes.
    file.read_to_end(&mut buffer).await?;

    println!("The bytes are: {:?}", &buffer.iter().map(|bt| char::from(bt.clone())).collect::<String>());

    Ok(())
}