use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Apartment {
    pub title: String,
    pub price: u32,
    pub quadrature: u32,
    pub rooms: String,
    pub floor: String,
    pub img: String,
    pub agent: String,
    pub price_history: Vec<(u64,u32)>,
    pub created_at: u64,
    pub updated_at: u64,
    pub closed_at: Option<u64>
}
