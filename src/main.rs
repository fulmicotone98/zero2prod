<<<<<<< HEAD
use zero2prod::run;
=======
use actix_web::{
    App, HttpRequest, HttpServer, Responder,
    web::{self},
};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
>>>>>>> 2fd370e9c4cbf0ce8c2da9fa720f8f4efc2f62f1

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
