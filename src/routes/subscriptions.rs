use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    tracing::info!("adding email='{}', name='{}' as new subscriber.", form.email, form.name);
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber saved.");
            HttpResponse::Ok().finish(
        )},
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
