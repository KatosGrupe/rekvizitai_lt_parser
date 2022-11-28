use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod entity;
mod scraper;
use clap::Parser;
use log::info;
use scraper::scrape_rekvizitai;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    listen_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    url: String,
}

async fn parse(item: web::Json<Request>) -> HttpResponse {
    info!("model: {:?}", &item);
    let result = scrape_rekvizitai(item.url.clone()).await;
    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/extractor").route(web::post().to(parse)))
    })
    .bind(args.listen_ip)?
    .run()
    .await
}
