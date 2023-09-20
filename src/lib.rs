mod entity;
mod scraper;

pub use entity::Entity;

pub async fn extract_data(url: &str) -> Entity {
    let html = scraper::download_page(url).await.unwrap();
    scraper::scrape_rekvizitai(&html).await
}

