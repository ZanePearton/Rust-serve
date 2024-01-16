use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(root_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn root_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello, World! This is a cool web test!")
    
}
