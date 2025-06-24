use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}