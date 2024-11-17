use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::io::{self, Write};
use sha2::{Sha256, Digest};

use anyhow::Result;

mod apartment;
use apartment::Apartment;

fn price_to_number(price: &str) -> Result<u32> {
    let number_str: String = price
        .chars()
        .filter(|c| c.is_digit(10)) // Keep only digits
        .collect();

    Ok(number_str.parse::<u32>()?)
}

fn quadrature_to_number(quadrature: &str) -> Option<u32> {
    // Split the string at the first '&' and take the part before it
    let part_before_ampersand = quadrature.split('&').next()?;

    // Extract digits only and parse into a u32
    let number: u32 = part_before_ampersand
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .ok()?;

    Some(number)
}

// Save the apartment to a JSON file
fn save_to_file(apartment: &Apartment, file_path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(apartment)?; // Serialize to JSON string
    let mut file = fs::File::create(file_path)?; // Create or overwrite the file
    file.write_all(json.as_bytes())?; // Write the JSON string to the file
    Ok(())
}

fn load_from_file(file_path: &str) -> Result<Apartment> {
    let json = fs::read_to_string(file_path)?; // Read file into string
    let apartment = serde_json::from_str(&json)?; // Deserialize JSON string into Apartment
    Ok(apartment)
}

fn get_file_name(title: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(title);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    println!("Hello, world!");
    let target_url = "https://www.halooglasi.com/nekretnine/izdavanje-stanova/beograd-zemun-zemunske-kapije?page=1";
    let response = reqwest::blocking::get(target_url);
    let html_content = response.unwrap().text().unwrap();
    //println!("{}", html_content);
    let document = scraper::Html::parse_document(&html_content);
    //println!("{:?}", document);
    let html_apartment_selector =
        scraper::Selector::parse(r#"div[class="col-md-12 col-sm-12 col-xs-12 col-lg-12"]"#)
            .unwrap();
    let apartments = document.select(&html_apartment_selector);
    //println!("{}", h1.inner_html());

    for apartment in apartments {
        // product title
        let link_selector = scraper::Selector::parse("a").unwrap();
        let title_selector = scraper::Selector::parse(r#"h3[class="product-title"]"#).unwrap();
        let title = apartment.select(&title_selector).next().unwrap();
        let title = title
            .select(&link_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned).unwrap();
        println!("title {}", title);
        // find price
        let price_selector =
            scraper::Selector::parse(r#"div[class="central-feature-wrapper"]"#).unwrap();
        let price_value_selector = scraper::Selector::parse("i").unwrap();
        let price = apartment
            .select(&price_selector)
            .next()
            .unwrap()
            .select(&price_value_selector)
            .next()
            .unwrap()
            .inner_html();
        let price = price_to_number(&price).unwrap();
        println!("price {}", price);

        // get features
        let features_selector =
            scraper::Selector::parse(r#"ul[class="product-features "]"#).unwrap();
        let features = apartment.select(&features_selector).next().unwrap();

        // square
        let feature_selector = scraper::Selector::parse(r#"div[class="value-wrapper"]"#).unwrap();
        let mut features = features.select(&feature_selector);

        let feature = features.next().unwrap();

        let quadrature = quadrature_to_number(&feature.inner_html()).unwrap();
        println!("quadrature {}", quadrature);

        // rooms
        let rooms = features.next().unwrap().inner_html();
        let rooms = rooms.split('&').next().unwrap();
        println!("rooms {}", rooms);

        // floor
        let floor = features.next().unwrap().inner_html();
        let floor = floor.split('&').next().unwrap();
        println!("floor {}", floor);

        // agencia - vlastnik
        let agent_selector =
            scraper::Selector::parse(r#"span[data-field-name="oglasivac_nekretnine_s"]"#).unwrap();
        let agent = apartment
            .select(&agent_selector)
            .next()
            .and_then(|a| a.value().attr("data-field-value"))
            .map(str::to_owned)
            .unwrap();

        println!("agent {}", agent);

        // img
        let img_selector = scraper::Selector::parse(r#"img[class=""]"#).unwrap();
        let img = apartment
            .select(&img_selector)
            .next()
            .unwrap()
            .attr("src")
            .unwrap();
        //.and_then(|img| img.value().attr("src"))
        //.map(str::to_owned).unwrap();

        println!("img {}", img);

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let apt = Apartment {
            title:title.to_owned(),
            price,
            quadrature,
            rooms: rooms.to_owned(),
            floor: floor.to_owned(),
            agent,
            img: img.to_owned(),
            price_history: Vec::new(),
            created_at: now_secs,
            updated_at: now_secs,
            closed_at: None,
        };

        let file_path = format!("{}.json",get_file_name(&title));

        // Save the apartment to a file
        save_to_file(&apt, &file_path).unwrap();
        println!("Apartment saved to {}", file_path);
    }
}
