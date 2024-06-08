use std::env;

use actix_files::Files;
use actix_web::{
    get,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

use dotenvy::dotenv;
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
    HttpServer::new(move || {
        print!("Starting server on port: {}", port);
        App::new()
            // .app_data(Data::new(pool.clone()))
            // .service(users::routes())
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
