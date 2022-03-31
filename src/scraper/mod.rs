extern crate image;
extern crate regex;
extern crate soup;
use crate::entity::Entity;
use crate::scraper::soup::NodeExt;
use crate::scraper::soup::QueryBuilderExt;
use image::GenericImageView;
use log::{debug, info, trace};
use regex::Regex;
use reqwest::Url;
use soup::Soup;

pub async fn scrape_rekvizitai(url: String) -> Entity {
    trace!("Scraping {}", url);
    let log_url = url.clone();
    let url = Url::parse(&url).unwrap();
    let document_string = reqwest::get(url).await.unwrap().text().await.unwrap();
    let soup = Soup::new(&document_string[..]);

    let mut tags = soup
        .tag("div")
        .attr("class", "info")
        .find()
        .unwrap()
        .tag("table")
        .find()
        .unwrap()
        .tag("tr")
        .find_all();

    let mut entity = Entity::new();

    let re = Regex::new(r#"src="([a-zA-Z0-9%/\.]+)""#).expect("Could not create regex parser");
    while let Some(tag) = tags.next() {
        let mut tags = tag.tag("td").find_all();

        while let Some(tag) = tags.next() {
            match tag.text().as_str() {
                "Adresas" => entity.address.push_str(tags.next().unwrap().text().trim()),
                "Atsiskaitomoji sąskaita" => entity
                    .account_number
                    .push_str(tags.next().unwrap().text().trim()),
                "Bankas" => entity.bank.push_str(tags.next().unwrap().text().trim()),
                "Darbo Laikas" => entity
                    .business_hours
                    .push_str(tags.next().unwrap().text().trim()),
                "Įmonės kodas" => entity
                    .registration_id
                    .push_str(tags.next().expect("Could not find next tag").text().trim()),
                "Mobilus telefonas" => {
                    let text = &tags.next().unwrap().display();
                    let text = re
                        .captures(text)
                        .expect("Could not get caputures")
                        .get(1)
                        .expect("No capture under index 0")
                        .as_str();
                    let text = format!("{}{}", "https://rekvizitai.vz.lt", text);
                    entity.mobile_phone.push_str(
                        &extract_text_from_url(Url::parse(&text).expect("Could not parse ze URL"))
                            .await,
                    )
                }
                "PVM mokėtojo kodas" => entity.vat_id.push_str(tags.next().unwrap().text().trim()),
                "Telefonas" => {
                    let text = &tags.next().unwrap().display();
                    let text = re
                        .captures(text)
                        .expect("Could not get caputures")
                        .get(1)
                        .expect("No capture under index 0")
                        .as_str();
                    let text = format!("{}{}", "https://rekvizitai.vz.lt", text);
                    entity.phone.push_str(
                        &extract_text_from_url(Url::parse(&text).expect("Could not parse ze URL"))
                            .await,
                    )
                }
                "Tinklalapis" => entity.website.push_str(tags.next().unwrap().text().trim()),
                "Vadovas" => entity.ceo.push_str(tags.next().unwrap().text().trim()),
                "Vidutinis atlyginimas" => entity
                    .average_wage
                    .push_str(tags.next().unwrap().text().trim()),
                _ => {}
            }
        }
    }
    let header = soup.tag("h1").attr("class", "fn").find().unwrap().text();
    let re = Regex::new(r#"(.+), ([A-ZĄČĘĖĮŠŲŪŽš]+)"#).expect("Could not create regex parser 2");
    for cap in re.captures_iter(&header) {
        entity.name.push_str(&cap[1]);
        entity.entity_type.push_str(&cap[2]);
    }
    let re = Regex::new(r#"([A-ZĄČĘĖĮŠŲŪŽš]+) "(.+)""#).expect("Could not create regex parser 2");
    for cap in re.captures_iter(&header) {
        entity.name.push_str(&cap[2]);
        entity.entity_type.push_str(&cap[1]);
    }

    info!("Entity (from url: {}): {:?}", log_url, entity);
    entity
}

async fn extract_text_from_url(url: Url) -> String {
    debug!("Image url: {}", url);
    let image = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let mut content = std::io::Cursor::new(image);
    {
        let mut out = std::fs::File::create("test.gif").unwrap();
        std::io::copy(&mut content, &mut out).unwrap();
    }

    let txt = extract_text_from_file("test.gif".to_string());
    trace!("Extracted from image: {}", txt);
    txt
}

fn extract_text_from_file(img: String) -> String {
    let img = image::open(img).unwrap();
    extract_text_from_image(img)
}

fn extract_text_from_image(img: image::DynamicImage) -> String {
    let mut new_img =
        image::DynamicImage::new_rgba8(100 + img.dimensions().0, 100 + img.dimensions().1);
    image::imageops::overlay(&mut new_img, &img, 50, 50);

    new_img.save("data/test.png").unwrap();

    let mut lt = leptess::LepTess::new(None, "lit").unwrap();
    lt.set_image("data/test.png").unwrap();
    lt.get_utf8_text().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::entity::Entity;
    #[test]
    fn scrape_rekvizitai() {
        let mut expected = Entity::new();
        expected
            .address
            .push_str("Žemaičių g. 28B, LT-44174 Kaunas");
        expected.average_wage.push_str("1 394,94 € (2019 m. gruodis) \n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tAtlyginimų istorija »");
        expected.ceo.push_str("Andrius Bagdonas, direktorius");
        expected.mobile_phone.push_str("+370 611 57390\n");
        expected.name.push_str("Katos grupė");
        expected.entity_type.push_str("KB");
        expected.phone.push_str("+370 37 440016\n");
        expected.registration_id.push_str("300028287");
        expected.vat_id.push_str("LT100001121613");
        expected.website.push_str("http://www.kata.lt");

        assert_eq!(
            expected,
            crate::scraper::scrape_rekvizitai(
                "https://rekvizitai.vz.lt/imone/katos_grupe".to_string()
            )
        );
    }

    #[test]
    fn scrape_rekvizitai_alt_header() {
        let mut expected = Entity::new();
        expected.account_number.push_str("LT247300010161303834");
        expected
            .address
            .push_str("Gargždupio g. 11, Gargždai, LT-96100 Klaipėdos r.");
        expected.ceo.push_str("Mantas Stalgys");
        expected.mobile_phone.push_str("+370 37 440016\n");
        expected.name.push_str("JOROMA");
        expected.entity_type.push_str("MB");
        expected.registration_id.push_str("305413988");
        // expected.vat_id.push_str("LT100001121613");
        // expected.website.push_str("http://www.kata.lt");

        assert_eq!(
            expected,
            crate::scraper::scrape_rekvizitai("https://rekvizitai.vz.lt/imone/joroma".to_string())
        );
    }
}
