use std::{collections::HashMap, fs::read_to_string};

use crypto::{digest::Digest, ed25519, hmac::Hmac, pbkdf2::pbkdf2, sha2::Sha256, sha2::Sha512};
use rand::{Rng, ThreadRng};

pub struct SeedPhrase<'a> {
    random: ThreadRng,
    word_map: HashMap<i32, &'a str>,
}

pub fn new()-> Result<'a, SeedPhrase> {
    println!("Hello new");
    let word_string = read_to_string("src\\bibs.txt").unwrap();
    let word_list: Vec<&str> = word_string.split("\r\n").collect();
    let mut word_map: HashMap<i32, &str> = HashMap::new();
    let mut i = 0;
    for word in &word_list {
        word_map.insert(i, &word);
        i += 1;
    }
    
    return SeedPhrase {random:rand::thread_rng() ,word_map };
}
impl SeedPhrase {
    //
}
