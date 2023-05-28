# Rust Web Server

A single threaded web server built **with NO frameworks** in Rust.

Requested for by my mentor: [Jim Nnamdi](https://github.com/jim-nnamdi).

# Discoveries
1. Rust `std` has a `net::TcpListener` that rivals Go's `http.ListenAndServe`. I used this.
