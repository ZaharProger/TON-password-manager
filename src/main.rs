pub mod seed_phrase;

fn main() {
    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    println!("{:?}", seed_phrase.get_keypair().1);
}
