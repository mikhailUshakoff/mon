use crate::apartment::Apartment;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct HalooglasiParser {
    target_url: String,
}

impl HalooglasiParser {
    pub fn new(target_url: String) -> Result<Self> {
        Ok(Self { target_url })
    }

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

    pub fn parse_page(&mut self) -> Result<Vec<Apartment>> {
        let mut result = Vec::new();
        let response = reqwest::blocking::get(&self.target_url);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let selector =
            scraper::Selector::parse(r#"div[class="col-md-12 col-sm-12 col-xs-12 col-lg-12"]"#)
                .unwrap();
        let apartments = document.select(&selector);

        println!("apartments ");
        for apartment in apartments {
            // product title
            let link_selector = scraper::Selector::parse("a").unwrap();
            let title_selector = scraper::Selector::parse(r#"h3[class="product-title"]"#).unwrap();
            let title = apartment.select(&title_selector).next().unwrap();
            let title = title
                .select(&link_selector)
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned)
                .unwrap();
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
            let price = Self::price_to_number(&price).unwrap();
            println!("price {}", price);

            // get features
            let features_selector =
                scraper::Selector::parse(r#"ul[class="product-features "]"#).unwrap();
            let features = apartment.select(&features_selector).next().unwrap();

            // square
            let feature_selector =
                scraper::Selector::parse(r#"div[class="value-wrapper"]"#).unwrap();
            let mut features = features.select(&feature_selector);

            let feature = features.next().unwrap();

            let quadrature = Self::quadrature_to_number(&feature.inner_html()).unwrap();
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
                scraper::Selector::parse(r#"span[data-field-name="oglasivac_nekretnine_s"]"#)
                    .unwrap();
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

            println!("img {}", img);

            //location
            let place_selector =
                scraper::Selector::parse(r#"ul[class="subtitle-places"]"#).unwrap();
            let li_selector = scraper::Selector::parse("li").unwrap();
            let place = apartment.select(&place_selector).next().unwrap();

            let mut place = place.select(&li_selector);

            let p1 = place.next().unwrap().inner_html();
            let p1 = p1.split('&').next().unwrap();
            println!("p1 {}", p1);

            let p2 = place.next().unwrap().inner_html();
            let p2 = p2.split('&').next().unwrap();
            println!("p2 {}", p2);

            let p3 = place.next().unwrap().inner_html();
            let p3 = p3.split('&').next().unwrap();
            println!("p3 {}", p3);

            let p4 = place.next().unwrap().inner_html();
            let p4 = p4.split('&').next().unwrap();
            println!("p4 {}", p4);

            let now_secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let apt = Apartment {
                title: title.to_owned(),
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
            result.push(apt);
        }
        Ok(result)
    }
}
