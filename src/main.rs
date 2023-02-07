use std::{fs::File, io::Write};

pub mod seed_phrase;
pub mod toncli_rust;

fn main() {
    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let private_key = seed_phrase.get_private_key();

    let file_name = "wallet\\build\\contract.pk";
    let mut file = File::create(file_name).unwrap();
    file.write_all(&private_key).ok(); 

    let toncli = toncli_rust::ToncliRust::new(
        toncli_rust::OStypes::Windows
    ).unwrap();

    let result = toncli.deploy_contract();
    println!("{}", result.data);
    println!("{}", result.message);
}
