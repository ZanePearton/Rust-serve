// Import necessary modules and types from the actix_web crate and standard library.
use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use std::env;

// The main function is marked with `actix_web::main`, which sets up an async runtime.
// This function will return a Result that, if an error occurs, will contain an `std::io::Error`.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set the environment variable for logging level to "info" for actix_web logs.
    env::set_var("RUST_LOG", "actix_web=info");
    // Initialize the env_logger logger, which will log information based on the RUST_LOG environment variable.
    env_logger::init();

    // Create and run an HTTP server.
    HttpServer::new(|| {
        // Initialize the Actix web application.
        App::new()
            // Add the Logger middleware to log all incoming requests.
            .wrap(Logger::default())
            // Define a route for the root URL ("/") that handles GET requests with the `root_handler` function.
            .route("/", web::get().to(root_handler))
    })
    // Bind the server to listen on the localhost address and port 8080.
    .bind("127.0.0.1:8080")?
    // Start the server and await its completion, handling any errors that occur.
    .run()
    .await
}

// Define an asynchronous handler function for the root URL.
// This function returns a type that implements the `Responder` trait, which can be converted into an HTTP response.
async fn root_handler() -> impl Responder {
    // Create an HTTP response with the status code 200 OK and the body "Hello, World! This is a cool web test!".
    HttpResponse::Ok().body("Hello, World! This is a cool web test!")
}
