use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in an Arc smart pointer
    let connection = Arc::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
