use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Initialize a server at the provided address.
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6666")
        .await
        .expect("Failed to initialize listener on the provided address");

    // Initialize a shared database.
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    // Wait for the incoming requests and then process them.
    loop {
        // Get the request.
        let (socket, _): (TcpStream, _) = listener
            .accept()
            .await
            .expect("Failed to establish a connection with a client");

        let db_for_task: Db = Arc::clone(&db);

        // Initialize a new task and process the request.
        tokio::spawn(async move {
            process(socket, db_for_task).await;
        });
    } // end loop
} // end main()

/// This function processes an incoming request.
///
async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // According to the design this variable is required to
    // to convert bytes into frames.
    let mut connection: Connection = Connection::new(socket);

    // Read frames to get the command from the receiver.
    while let Some(frame) = connection
        .read_frame()
        .await
        .expect("Failed to read a frame")
    {
        let response = match Command::from_frame(frame).expect("Failed to identify a command") {
            Set(cmd) => {
                // Lock the database to perform an operation on it.
                let mut db = db
                    .lock()
                    .expect("Failed to get the control over the database");
                db.insert(cmd.key().to_string(), Bytes::copy_from_slice(cmd.value()));
                Frame::Simple("OK".to_string())
            } // end Set
            Get(cmd) => {
                // Lock the database to perfrom an operation on it.
                let db = db
                    .lock()
                    .expect("Failed to get the control over the database");
                // Determine if the value is set or not.
                if let Some(val) = db.get(cmd.key()) {
                    // The value is found.
                    Frame::Bulk(val.clone().into())
                } else {
                    // The value is not found.
                    Frame::Null
                } // end if
            } // end Get
            cmd => panic!("Failed to identify the command: {:?}", cmd),
        }; // end match

        // Write the response to the client.
        connection
            .write_frame(&response)
            .await
            .expect("Failed to write the response for a client");
    } // end while
} // end process()
