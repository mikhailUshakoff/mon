use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Apartment {
    pub title: String,
    pub price: u32,
    pub quadrature: u32,
    pub rooms: String,
    pub floor: String,
    pub img: String,
    pub agent: String,
    pub price_history: Vec<(u64, u32)>,
    pub created_at: u64,
    pub updated_at: u64,
    pub closed_at: Option<u64>,
}

impl Apartment {
    pub fn get_file_name(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.title);
        let result = hasher.finalize();
        format!("{:x}.json", result)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?; 
        let mut file = fs::File::create(file_path)?; 
        file.write_all(json.as_bytes())?; 
        Ok(())
    }

    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let contents = fs::read_to_string(file_path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}
