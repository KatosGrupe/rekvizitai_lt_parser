use crate::entity::Entity;
// use image::GenericImageView;
use log::{debug, info, trace};
use regex::Regex;
use reqwest::Url;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Failed to parse provided url to download")]
    UrlParse(#[from] url::ParseError),
}

pub async fn download_page(url: &str) -> Result<String, DownloadError> {
    let url = Url::parse(url)?;
    let document_string = reqwest::get(url).await.unwrap().text().await.unwrap();
    Ok(document_string)
}

pub async fn scrape_rekvizitai(data: &str) -> Entity {

    let mut result = Entity::new();
    use scraper::Html;
    use scraper::Selector;
    let document = Html::parse_document(data);
    let products_selector = Selector::parse("td[class=name]").expect("Products selector expected to parse correctly");
    let values_selector = Selector::parse("td[class=value]").expect("Values selector expected to parse correctly");
    let image_selector = Selector::parse("img").expect("Image selector expected to parse correctly");
    for node in document.select(&products_selector)
                        .into_iter()
                        .zip(document.select(&values_selector)) {
        match node.0.inner_html().as_ref() {
            "Įmonės kodas" => result.registration_id = node.1.inner_html().trim().to_string(),
            "PVM mokėtojo kodas" => result.vat_id = node.1.inner_html().trim().to_string(),
            "Vadovas" => result.ceo = node.1.inner_html().trim().to_string(),
            "Adresas" => result.address = node.1.inner_html().trim().split('\n').nth(0).unwrap_or("").to_string(),
            "Telefonas" => {
                result.phone = match node.1.select(&image_selector).nth(0) {
                    Some(val) => {
                        let src = val.value().attr("data-cfsrc")
                                             .expect("Img tag found but it's missing data-cfsrc attribute (Update scraper code required)");
                        let url = format!("https://rekvizitai.vz.lt{}", src);
                        download_and_extract_text(&url).await
                    }
                    None => "".to_string()
                };
            }
            "Mobilus telefonas" => {
                result.mobile_phone = match node.1.select(&image_selector).nth(0) {
                    Some(val) => {
                        let src = val.value().attr("data-cfsrc")
                                             .expect("Img tag found but it's missing data-cfsrc attribute (Update scraper code required)");
                        let url = format!("https://rekvizitai.vz.lt{}", src);
                        download_and_extract_text(&url).await
                    }
                    None => "".to_string()
                };
            }
            "Faksas" => {}
            "El. pašto adresas" => {}
            "Tinklalapis" => {
                let url_selector = Selector::parse("a").expect("Url selector expected to parse correctly");
                result.website = match node.1.select(&url_selector).nth(0) {
                    Some(val) => val.value().attr("href")
                                            .expect("<a> tag found but it's missing href attribute (Update scraper code required)"),
                    None => ""
                }.to_string();
            }
            x => println!("{} is unhandled: {:?}", x, node.1.inner_html().trim()),
        }
    }

    result
}

pub async fn download_and_extract_text(url: &str) -> String {
    info!("Extracting url: {url}");
    let dir = tempfile::tempdir().unwrap();

    let image_bytes = reqwest::get(url).await
                                       .expect("Failed to reach url")
                                       .bytes()
                                       .await
                                       .expect("Failed to download image");
    let orig_image_path = dir.path().join("orig_image.gif");
    let mut orig_image_file = File::create(orig_image_path.clone())
        .await
        .expect("Failed to create image file to write to");
    orig_image_file.write_all(&image_bytes).await.expect("Failed to save orig image");

    info!("Increasing dimensions for better OCR recognition for {url}");
    let dimensions = image::image_dimensions(orig_image_path.clone()).expect("Failed to get image dimensions");
    let old_img = image::io::Reader::open(orig_image_path)
        .expect("Failed to open downloaded image")
        .decode()
        .expect("Failed to decode downloaded image");
    let mut new_img =
        image::DynamicImage::new_rgba8(100 + dimensions.0, 100 + dimensions.1);
    image::imageops::overlay(&mut new_img, &old_img, 50, 50);

    let new_image_path = dir.path().join("new_image.png");
    new_img.save(new_image_path.clone()).expect("Failed to save edited image");

    info!("Running OCR for {url}");
    let mut lt = leptess::LepTess::new(None, "lit").expect("Failed to load 'lit' leptess data model");
    lt.set_image(new_image_path).expect("Failed to set image path for OCR");

    let result = lt.get_utf8_text().expect("Failed to extract utf-8 data");
    dir.close().expect("Failed to drop temp directory and free hard disk");
    result.trim().to_string()
}
