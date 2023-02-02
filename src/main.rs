use rand::Rng;
fn main() {
    
    let random_bytes: Vec<u8> = (0..256)
        .map(|_| { rand::thread_rng().gen_range(0, 2) })
        .collect();
    println!("{:?}", random_bytes);
}
