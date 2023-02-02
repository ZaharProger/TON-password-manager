use std::{fs, collections::HashMap};

use rand::Rng;
use crypto::{sha2::Sha256, digest::Digest};

fn main() {
    //Инициализируем генератор рандома
    let mut random = rand::thread_rng();

    //Читаем слова с вордлиста и формируем словарь
    let word_string = fs::read_to_string("src\\bibs.txt").unwrap();
    let word_list: Vec<&str> = word_string.split("\r\n").collect();
    let mut word_map: HashMap<i32, &str> = HashMap::new();

    let mut i = 0;
    for word in &word_list {
        word_map.insert(i, &word);
        i += 1;
    }

    //Генерируем энтропию
    let mut entropy = String::new();
    for _ in 0..256 {
        entropy.push_str(&random.gen_range(0, 2).to_string());
    }
    println!("Исходная энтропия: {}", &entropy);

    //Получаем хэш по энтропии
    let mut sha256 = Sha256::new();
    let mut hash: Vec<u8> = vec![0; 32];

    sha256.input_str(&entropy);
    sha256.result(&mut hash);

    //Создаем бинарную строку из хэша
    let mut binary_hash = String::new();
    for byte in &hash {
        binary_hash.push_str(&format!("{:08b}", byte));
    }

    //Добавляем контрольную сумму к энтропии
    entropy.push_str(&binary_hash[..8]);
    println!("Окончательная энтропия: {}", &entropy);

    //Формируем мнемоническую фразу
    let mut mnemonic: Vec<&str> = Vec::new();
    let mut bunch_start = 0;
    let mut bunch_end = 11;
    
    while bunch_start != entropy.len() {
        let entropy_slice = &entropy[bunch_start..bunch_end];
        let word_index = isize::from_str_radix(entropy_slice, 2);
        mnemonic.push(word_map[&(word_index.unwrap() as i32)]);

        bunch_start = bunch_end;
        bunch_end += entropy_slice.len();
    }
    
    println!("Мнемоника: {:?}", mnemonic);

}
