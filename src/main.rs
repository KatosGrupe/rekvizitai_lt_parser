use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod entity;
mod scraper;
use clap::Parser;
use log::info;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

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
    let html = scraper::download_page(&item.url).await.unwrap();
    let result = scraper::scrape_rekvizitai(&html).await;
    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let source = {
        let mut result = String::new();
        let mut f = File::open(format!("rekvizitai.html")).await.unwrap();
        f.read_to_string(&mut result).await.unwrap();
        result
    };

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/extractor").route(web::get().to(parse)))
    })
    .bind(args.listen_ip)?
    .run()
    .await
}
