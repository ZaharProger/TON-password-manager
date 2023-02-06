pub mod seed_phrase;

fn main() {
    let mut seedPhrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    println!("{:?}", seedPhrase.Get_keypair().1);
}
