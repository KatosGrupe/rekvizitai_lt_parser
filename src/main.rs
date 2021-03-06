use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod entity;
mod scraper;
use log::info;
use scraper::scrape_rekvizitai;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    url: String,
}

async fn parse(item: web::Json<Request>) -> HttpResponse {
    info!("model: {:?}", &item);
    let result = scrape_rekvizitai(item.url.clone());
    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web,rekvizitai_lt_parser=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/extractor").route(web::post().to(parse)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
