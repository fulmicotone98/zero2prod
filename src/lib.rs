use actix_web::{
    web::{self}, App, HttpResponse, HttpServer, Responder
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    // HttpServer is the backbone of our application.
    // Handles all the transport layer of our application.
    HttpServer::new(|| {
        // After HttpServer has established a new connection with a client,
        // App start handling all the request to the APIs.
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}