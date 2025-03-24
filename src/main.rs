use actix_web::{
    web::{self}, App, HttpRequest, HttpServer, Responder
};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // HttpServer is the backbone of our application.
    // Handles all the transport layer of our application.
    HttpServer::new(|| {
        // After HttpServer has established a new connection with a client,
        // App start handling all the request to the APIs.
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
