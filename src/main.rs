mod apartment;

mod halooglasi_parser;
use halooglasi_parser::HalooglasiParser;

fn main() {
    let mut parser = HalooglasiParser::new("https://www.halooglasi.com/nekretnine/izdavanje-stanova/beograd-zemun-zemunske-kapije?page=1".to_string()).unwrap();

    let apartments = parser.parse_page().unwrap();
    for apartment in apartments {
        let file_path = apartment.get_file_name();
        println!("Apartment saved to {}", file_path);
        apartment.save_to_file(&file_path).unwrap();
    }
}
