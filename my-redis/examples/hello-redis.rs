use bytes::Bytes;
use mini_redis::{
    client::{self, Client},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Open the connection to the mini-redis address.
    let mut client: Client = client::connect("127.0.0.1:6666").await?;

    // Set the key 'hello' with value 'world'.
    client.set("hello", "world".into()).await?;

    // Get key 'hello'.
    let result: Option<Bytes> = client.get("hello").await?;

    println!("Got value from the server; Result is {:?}", result);

    Ok(())
} // end main()
