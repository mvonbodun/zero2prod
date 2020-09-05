use actix_web::{web, HttpResponse};
#[allow(unused_imports)]
use chrono::Utc;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;
#[allow(unused_imports)]
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Subscribe to email
#[allow(clippy::toplevel_ref_arg)]
pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<Arc<PgPool>>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref().deref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
