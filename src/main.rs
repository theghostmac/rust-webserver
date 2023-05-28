use std::net::TcpListener;
use std::fs;
use toml::Value;

fn main() {
    // handle the config.toml file parsing.
    let config_contents = fs::read_to_string("config.toml").expect("Failed to read the config.toml file...");
    let config: Value = config_contents.parse().expect("Failed to parse the config.toml file contents");
    // set what to expect if parsing fails.
    let _port = config["port"].as_integer().unwrap_or(5000) as u16;
    let _root_directory = config["root_directory"].as_str().unwrap_or("public");

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
