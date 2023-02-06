use std::{fs::File, io::Write};

pub mod seed_phrase;

fn main() {
    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let (private_key, public_key) = seed_phrase.get_keypair();

    let (new_private_key, new_public_key) = crypto::ed25519::keypair(&private_key);

    let file_name = "contract.pk";
    let mut file = File::create(file_name).unwrap();
    file.write_all(&new_private_key[..32]); 

    println!("Old private: {:?}", private_key);
    println!("Old public: {:?}", public_key);
    println!("New private: {:?}", new_private_key);
    println!("New public: {:?}", new_public_key);
}
