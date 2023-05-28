# Rust Web Server (rust-webserver)
This is a basic implementation of a web server using Rust programming language. 
The purpose of this project is for me to demonstrate I can build a simple web 
server without relying on any Rust frameworks. 
It serves as a starting point for understanding the fundamentals 
of building a file upload and management service in Rust.

## Current Features

- HTTP server capable of handling GET requests
- Minimal dependencies
- Extensible and customizable `cargo.toml` file

## Future Features
⚠️ See [Changelog file](CHANGELOG.md).

## Requirements

- Rust programming language (version 1.54.0 or higher)

## Getting Started

To get started with the Rust Web Server, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/theghostmac/rust-webserver.git
   ```

2. Change to the project directory:

   ```bash
   cd rust-webserver
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the server:

   ```bash
   cargo run
   ```

5. Access the server in your web browser:

   ```
   http://localhost:8000
   ```
   Or if you use a Jetbrains IDE, click the Play ▶️ button on the [http](check.http) file.
   The server is now running, and you should see a simple "Hello, World!" message in your browser.

## Configuration

The server can be configured by modifying the `config.toml` file. It allows you to specify the server's port number and the root directory for serving static files. By default, the server runs on port 8000 and serves static files from the `public` directory.

## Adding Routes

To add custom routes to the server, you need to modify the `main.rs` file. The server uses a simple routing mechanism based on the requested path. You can add your custom logic to handle specific routes and return appropriate responses.

## Contributing

Contributions to the Rust Web Server project are welcome! 
If you find any issues or have suggestions for improvements, 
please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as per the terms of the license.

## Acknowledgements

This project was requested for by my mentor: [Jim Nnamdi](https://github.com/jim-nnamdi).

## Contact

If you have any questions or need further assistance with this project, feel free to contact the project maintainer at [your-email@example.com](mailto:your-email@example.com).

Happy coding!