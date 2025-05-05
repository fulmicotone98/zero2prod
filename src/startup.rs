use crate::routes::{health_check::*, subscriptions::*};
use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer is the backbone of our application.
    // Handles all the transport layer of our application.

    let server = HttpServer::new(|| {
        // After HttpServer has established a new connection with a client,
        // App start handling all the request to the APIs.
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    //.await

    /* When we call .await, the server starts listening incoming requests and handles them, but it will never shutdown. .awaits implies that spawn_app() in the test will never return since the server never ends. */

    Ok(server)
}
