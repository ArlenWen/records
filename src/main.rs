use chrono::{DateTime, Utc};
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Json;
use lazy_static::lazy_static;
use serde::Serialize;

mod routes;
mod logger;
mod conf;

lazy_static! {
    pub static ref START_TIME: DateTime<Utc> = Utc::now();
}

#[derive(Serialize)]
pub struct Health {
    start_time: String,
    running_time: i64,
}

async fn status() -> Json<Health> {
    let num_sec = Utc::now().time() - START_TIME.time();
    let body = Health {
        start_time: START_TIME.format("%Y-%m-%d %H:%M:%S").to_string(),
        running_time: num_sec.num_seconds(),
    };
    Json(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config = conf::parse_config(conf::read_conf_file());
    let service_config = server_config.service;
    let log_config = server_config.log;
    logger::init_log(&log_config);

    HttpServer::new(
        || {App::new()
            .wrap(Logger::default())
            .service(web::scope("/service").route("/health", web::get().to(status)))})
        .bind((service_config.host, service_config.port))?
        .run()
        .await
}