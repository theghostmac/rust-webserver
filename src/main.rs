use std::net::TcpListener;

fn main() {
    // create a listener and bind to a port.
    let server_listener = TcpListener::bind("127.0.0.1:5000")
        .unwrap();

    // listen to incoming streams from TcpListener.
    for incoming_streams in server_listener.incoming() {
        // calling unwrap to terminate the server when there is an error.
        let _incoming_streams = incoming_streams.unwrap();
        // server message...
        println!("Successful connection to server...")
    }
}
