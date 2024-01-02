
```markdown
# Rust-Server

This simple web server, written in Rust, responds with "Hello, World! This is a cool web server!" to all requests.

## Description

Rust-Server is a basic project designed to provide an understanding of the basics of web serving in Rust using Hyper and Tokio. It listens on port 8080 and serves a straightforward greeting message, perfect for individuals looking to dive into web development with Rust.

## Getting Started

### Dependencies

- Rust programming language
- Cargo (Rust's package manager)

### Installing

Clone the repository to your local machine:

```bash
git clone https://github.com/YourUsername/Rust-server.git
```

### Running the Server

Navigate to the directory where you cloned the repo and run:

```bash
cargo run
```

Now, you can visit `http://localhost:8080` in your browser to see the server running.

## Understanding the Code

- **Cargo.toml**: Configuration file for Rust's package manager, defining project details and dependencies.
- **src/main.rs**: Contains the server code.
  - `async fn hello_world`: An asynchronous function that handles HTTP requests and returns a greeting message.
  - `#[tokio::main]`: Macro to define the entry point of the async application.
  - `Server::bind(&addr).serve(make_svc)`: Binds the server to the specified address and starts serving using the service defined.
  - `graceful.with_graceful_shutdown(shutdown_signal())`: Allows the server to shut down gracefully upon receiving a system signal.
- **Shutdown Signal Handling**: Using Tokio's signal handling to listen for termination or interrupt signals for a graceful shutdown.


## Authors

- **[Zane Pearton](https://github.com/zanepearton)** - Initial work.
```
