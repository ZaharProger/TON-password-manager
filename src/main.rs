pub mod seed_phrase;
pub mod toncli_rust;

fn main() {
    let toncli = toncli_rust::ToncliRust::new(
        toncli_rust::OStypes::Windows
    ).unwrap();

    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let result = toncli.deploy_contract(&seed_phrase.get_private_key());
    
    println!("{}", result.result);
    println!("{}", result.data);
    println!("{}", result.message);
}
