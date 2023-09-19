use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod entity;
mod scraper;
use clap::Parser;
use log::info;
use openssl::ssl::SslAcceptor;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    listen_ip: String,
    #[arg(short, long, default_value = "key.pem")]
    key: String,
    #[arg(short, long, default_value = "cert.pem")]
    cert: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    url: String,
}

async fn parse(args: actix_web::web::Query<Request>) -> HttpResponse {
    info!("model: {:?}", &args);
    let html = scraper::download_page(&args.url).await.unwrap();
    let result = scraper::scrape_rekvizitai(&html).await;
    HttpResponse::Ok().json(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(args.key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(args.cert).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/extractor").route(web::get().to(parse)))
    })
    .bind_openssl(args.listen_ip, builder)?
    .run()
    .await
}
