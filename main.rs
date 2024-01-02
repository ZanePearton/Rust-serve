use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::signal::unix::{signal, SignalKind};
use std::convert::Infallible;
use std::net::SocketAddr;

// Define your HTTP server handler.
async fn hello_world(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, World! This is a cool web server!")))
}

#[tokio::main]
async fn main() {
    // Set the address to run our socket on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create a service that will handle the connection.
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(hello_world)) }
    });

    // Create the server.
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server with graceful shutdown.
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening on http://{}", addr);

    // Await the hyper server and shutdown signal.
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

// Provides a future that completes when the system receives a SIGINT or SIGTERM.
async fn shutdown_signal() {
    // Create channels to listen for the signal.
    let mut term_signal = signal(SignalKind::terminate()).expect("Could not bind to the signal.");
    let mut int_signal = signal(SignalKind::interrupt()).expect("Could not bind to the signal.");

    // Listen for either signal.
    tokio::select! {
        _ = term_signal.recv() => println!("Received SIGTERM, initiating shutdown."),
        _ = int_signal.recv() => println!("Received SIGINT, initiating shutdown."),
    }
    println!("Shutting down server");
}
