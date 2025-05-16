use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// From the application data of our app we can get the connection to the Postgres DB
pub async fn subscribe(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> HttpResponse {
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
    .map(|_| HttpResponse::Ok().finish())
    .unwrap_or_else(|e| {
        println!("Failed to execute query: {e}");
        HttpResponse::InternalServerError().finish()
    })
}
