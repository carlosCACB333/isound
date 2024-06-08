use actix_web::{get, web, HttpResponse, Responder};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    q: String,
}

#[get("/search")]
pub async fn search_videos(query: web::Query<Query>) -> impl Responder {
    let q: String = query.q.clone();
    let api_key = std::env::var("YOUTUBE_API_KEY").unwrap();
    let url = format!(
        "https://youtube.googleapis.com/youtube/v3/search?part=snippet&part=id&maxResults=25&type=video&videoCategoryId=10&q={}&key={}",
        q, api_key
    );

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client.get(&url).send().await.unwrap();
    let results = response.json::<serde_json::Value>().await.unwrap();

    HttpResponse::Ok().json(results)
}

pub fn routes() -> actix_web::Scope {
    web::scope("/videos").service(search_videos)
}
