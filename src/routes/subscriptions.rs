use actix_web::{HttpResponse, web};
use log;
use sqlx::PgPool;
#[allow(unused_imports)]
use sqlx::types::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// From the application data of our app we can get the connection to the Postgres DB
pub async fn subscribe(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();

    log::info!(
        "req_id: {} - Adding {}, {} as new subscriber",
        req_id,
        form.email,
        form.name
    );
    log::info!("req_id: {} - Saving new subscriber in the database", req_id);
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        time::OffsetDateTime::now_utc() // UTC time -> No offset
    )
    // get_ref to get an immutable reference to the PgPool, which is inside an Arc thank to web::Data
    .execute(pg_pool.as_ref())
    .await
    .map(|_| {
        log::info!("req_id: {} - New subscriber info has been saved", req_id);
        HttpResponse::Ok().finish()
    })
    .unwrap_or_else(|e| {
        log::error!("req_id: {} - Failed to execute query: {:?}", req_id, e);
        HttpResponse::InternalServerError().finish()
    })
}
