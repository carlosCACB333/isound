use actix_files::Files;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;
use std::env;

// mod schema;
// mod tools;
// mod users;
mod videos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stage = env::var("STAGE").unwrap_or("development".to_string());
    if stage == "development" {
        dotenv().ok();
    }
    let port = u16::from_str_radix(&std::env::var("APP_PORT").unwrap(), 10).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    HttpServer::new(move || {
        info!("Starting server on port {}", port);
        App::new()
            // .app_data(Data::new(pool.clone()))
            // .service(users::routes())
            .wrap(Logger::default())
            .service(videos::routes())
            .service(Files::new("/static", "static").show_files_listing())
            .service(web::scope("").service(home_page))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[get("/")]
pub async fn home_page() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the home page!")
}
