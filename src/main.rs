use std::net::TcpListener;


fn main() {
    // create a listener similar to a new server mux.
    let serverListener = TcpListener::bind("127.0.0.1:5000")
        .unwrap();

    // listen to incoming streams from TcpListener
    for incomingStreams in serverListener.incoming() {
        let incomingStreams = incomingStreams.unwrap();

        println!("Successful connection to server...")
    }
}
