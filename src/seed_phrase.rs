use std::{collections::HashMap, fs::read_to_string, fmt::Error};

use crypto::{digest::Digest, ed25519, hmac::Hmac, 
    pbkdf2::pbkdf2, sha2::Sha256, sha2::Sha512};
use rand::{Rng, ThreadRng};

pub struct SeedPhrase {
    random: ThreadRng,
    word_map: HashMap<i32, String>,
}

impl SeedPhrase {
    pub fn new()-> Result<SeedPhrase, Error> {
        println!("Hello new");

        let word_string = read_to_string("src\\bibs.txt").unwrap();
        let word_list: Vec<&str> = word_string.split("\r\n").collect();
        let mut word_map: HashMap<i32, String> = HashMap::new();

        let mut i = 0;
        for word in &word_list {
            word_map.insert(i, word.to_string());
            i += 1;
        }
        
        return Ok(SeedPhrase {
            random: rand::thread_rng(),
            word_map,
        })
    }
}
