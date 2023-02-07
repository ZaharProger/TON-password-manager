use std::{fs::File, io::Write};

pub mod seed_phrase;

fn main() {
    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let (private_key, public_key) = seed_phrase.get_keypair();

    let file_name = "wallet\\build\\contract.pk";
    let mut file = File::create(file_name).unwrap();

    file.write_all(&private_key).ok(); 

    println!("Old private: {:?}", private_key);
    println!("Old public: {:?}", public_key);
}
