use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

mod crypto;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("IronPass Backend is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(web::resource("/health").to(health))
    })
    .bind((host, port))?
    .run()
    .await
}