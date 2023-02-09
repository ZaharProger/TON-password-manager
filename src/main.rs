pub mod seed_phrase;
pub mod toncli_rust;

fn main() {
    let toncli = toncli_rust::ToncliRust::new(
        toncli_rust::OStypes::Windows
    ).unwrap();

    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let private_key = seed_phrase.get_private_key();
    let result = toncli.deploy_contract(&private_key);
    
    println!("{}", result.result);
    println!("{}", result.data);
    println!("{}", result.message);

    let contract_address = toncli.get_contract_address(&private_key);
    println!("{contract_address}");
}
