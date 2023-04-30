use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Establish a listener at a specific address.
    let socket: TcpStream = TcpStream::connect("127.0.0.1:6666").await?;

    // Split the reader and writer streams.
    let (mut rd, mut wr) : (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = io::split(socket);

    // Write data in the background.
    tokio::spawn(async move {
        wr.write_all(b"Hello,").await.expect("Failed to write the data to a stream");
        wr.write_all(b"world!").await.expect("Failed to write the data to a steam");
    });

    let mut buf: Vec<u8> = vec![0; 128];

    // Listen for the connections
    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        } // end if

        println!("GOT = {:?}", &buf[..n]);
    } // end loops

    Ok(())
}