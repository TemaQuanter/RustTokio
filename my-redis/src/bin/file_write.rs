use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Open an existing file.
    let mut file = OpenOptions::new().append(true).open("foo.txt").await?;

    let n = file
        .write(b"Sweet dreams are made of this, who am I to disagree")
        .await?;

    println!("Manages to write {n} bytes to the file");
    Ok(())
} // end main()
