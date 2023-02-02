use rand::Rng;
fn main() {
    let mut random = rand::thread_rng();
    let mut random_bytes = String::from("");
    for _ in 0..256 {
        random_bytes += &random.gen_range(0, 2).to_string();
    }
    println!("{}", random_bytes);
}
