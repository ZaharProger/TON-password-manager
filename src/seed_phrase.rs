use std::{collections::HashMap, fmt::Error, fs::read_to_string};

use crypto::{digest::Digest, hmac::Hmac, pbkdf2::pbkdf2, 
    sha2::Sha256, sha2::Sha512, curve25519::ge_scalarmult_base};
use rand::{Rng, ThreadRng};

pub struct SeedPhrase {
    random: ThreadRng,
    word_map: HashMap<i32, String>,
    entropy: Vec<u8>,
    iterations: u32,
}

impl SeedPhrase {
    //Создает экземпляр класса
    pub fn new(itt: u32) -> Result<SeedPhrase, Error> {
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
            entropy: Vec::new(),
            iterations: itt
        });
    }
    //Возвращает окончательную энтропию
    fn create_entropy(&mut self) {
        self.entropy = (0..256)
            .map(|_| self.random.gen_range(0, 2))
            .collect();

        let mut sha256 = Sha256::new();
        let mut buffer: Vec<u8> = vec![0; 32];

        sha256.input(&self.entropy);
        sha256.result(&mut buffer);

        let first_byte = format!("{:08b}", buffer[0]);
        for bit in first_byte.chars() {
            self.entropy.push(bit.to_digit(2).unwrap() as u8);
        }
    }
    //Возвращает мнемоническую фразу
    pub fn mnemonic(&mut self) -> Vec<String> {
        let mut mnemonic: Vec<String> = Vec::new();
        let mut bunch_start = 0;
        let mut bunch_end = 11;

        self.create_entropy();

        while bunch_start != self.entropy.len() {
            let entropy_slice: String = self.entropy[bunch_start..bunch_end]
                .iter()
                .map(|entropy_digit| entropy_digit.to_string())
                .collect();

            let word_index = isize::from_str_radix(&entropy_slice, 2);
            mnemonic.push(self.word_map[&(word_index.unwrap() as i32)].to_string());

            bunch_start = bunch_end;
            bunch_end += entropy_slice.len();
        }

        return mnemonic;
    }
    //Сид фраза
    fn seed(&mut self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 32];
        let salt: Vec<u8> = (0..128)
            .map(|_| self.random.gen_range(0, 2))
            .collect();
            
        pbkdf2(
            &mut Hmac::new(Sha512::new(), &self.entropy),
            &salt,
            self.iterations,
            &mut buffer,
        );

        return buffer;
    }
    //Получение пары ключей(0-Приватный 1-Публичный)
    pub fn get_private_key(&mut self) -> [u8; 32] {
        let seed = self.seed();

        let mut private_key: [u8; 32] = [0; 32];
        let mut hasher = Sha256::new();

        hasher.input(&seed);
        hasher.result(&mut private_key);
        private_key[0] &= 248;
        private_key[31] &= 63;
        private_key[31] |= 64;

        let scalar_base = ge_scalarmult_base(&private_key);
        let public_key = scalar_base.to_bytes();
        
        for (dest, src) in (&mut private_key).iter_mut().zip(public_key.iter()) {
            *dest = *src;
        }
        for (dest, src) in (&mut private_key).iter_mut().zip(seed.iter()) {
            *dest = *src;
        }

        return private_key;
    }
}
