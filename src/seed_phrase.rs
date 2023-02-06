use std::{collections::HashMap, fmt::Error, fs::read_to_string};

use crypto::{digest::Digest, ed25519, hmac::Hmac, pbkdf2::pbkdf2, sha2::Sha256, sha2::Sha512};
use rand::{Rng, ThreadRng};

pub struct SeedPhrase {
    random: ThreadRng,
    word_map: HashMap<i32, String>,
}

impl SeedPhrase {
    //Создает экземпляр класса
    pub fn new() -> Result<SeedPhrase, Error> {
        println!("Hello new");

        let word_string = read_to_string("bibs.txt").unwrap();
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
        });
    }
    //Возвращает окончательную энтропию
    fn entropy(&mut self) -> Vec<u8> {
        let mut entropy: Vec<u8> = (0..256)
        .map(|_| self.random.gen_range(0, 2))
        .collect();

        let mut sha256 = Sha256::new();
        let mut buffer: Vec<u8> = vec![0; 32];

        sha256.input(&entropy);
        sha256.result(&mut buffer);

        let first_byte = format!("{:08b}", buffer[0]);
        for bit in first_byte.chars() {
            entropy.push(bit.to_digit(2).unwrap() as u8);
        }

        return entropy;
    }
    //Возвращает мнемоническую фразу
    pub fn mnemonic(&mut self) -> Vec<String> {
        let mut mnemonic: Vec<String> = Vec::new();
        let mut bunch_start = 0;
        let mut bunch_end = 11;

        let entropy = self.entropy();

        while bunch_start != entropy.len() {
            let entropy_slice: String = entropy[bunch_start..bunch_end]
                .iter()
                .map(|entropy_digit| entropy_digit.to_string())
                .collect();
                
            let word_index = isize::from_str_radix(&entropy_slice, 2);
            mnemonic.push(self.word_map[&(word_index.unwrap() as i32)].to_string());

            bunch_start = bunch_end;
            bunch_end += entropy_slice.len();
        }

        return mnemonic
    }

}
