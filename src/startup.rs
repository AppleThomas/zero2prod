use crate::routes::{
    subscribe, health_check
};
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || { 
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server) 
}