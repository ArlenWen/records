use chrono::{DateTime, FixedOffset, Local, Utc};
use actix_web::{web, App, HttpResponse, HttpServer};

mod routes;
mod logger;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(
        || App::new()
            .service(web::scope("/service").to(|| HttpResponse::Ok())))
        .bind("0.0.0.0:7005")?
        .run()
        .await
}