
mod seed_phrase;

fn main() {

}
// fn main() {
//     //Инициализируем генератор рандома
//     let mut random = rand::thread_rng();

//     //Читаем слова с вордлиста и формируем словарь
//     let word_string = read_to_string("src\\bibs.txt").unwrap();
//     let word_list: Vec<&str> = word_string.split("\r\n").collect();
//     let mut word_map: HashMap<i32, &str> = HashMap::new();

//     let mut i = 0;
//     for word in &word_list {
//         word_map.insert(i, &word);
//         i += 1;
//     }

//     //Генерируем энтропию
//     let mut entropy: Vec<u8> = (0..256)
//         .map(|_| random.gen_range(0, 2))
//         .collect();

//     println!("Исходная энтропия: {:?}", &entropy);

//     //Получаем хэш по энтропии
//     let mut sha256 = Sha256::new();
//     let mut buffer: Vec<u8> = vec![0; 32];

//     sha256.input(&entropy);
//     sha256.result(&mut buffer);

//     //Добавляем контрольную сумму к энтропии
//     let first_byte = format!("{:08b}", buffer[0]);
//     for bit in first_byte.chars() {
//         entropy.push(bit.to_digit(2).unwrap() as u8);
//     }
//     println!("Окончательная энтропия: {:?}", &entropy);

//     //Формируем мнемоническую фразу
//     let mut mnemonic: Vec<&str> = Vec::new();
//     let mut bunch_start = 0;
//     let mut bunch_end = 11;

//     while bunch_start != entropy.len() {
//         let entropy_slice: String = entropy[bunch_start..bunch_end]
//             .iter()
//             .map(|entropy_digit| entropy_digit.to_string())
//             .collect();
//         let word_index = isize::from_str_radix(&entropy_slice, 2);
//         mnemonic.push(word_map[&(word_index.unwrap() as i32)]);

//         bunch_start = bunch_end;
//         bunch_end += entropy_slice.len();
//     }

//     println!("Мнемоника: {:?}", mnemonic);

//     //Генерируем Seed через PBKDF2
//     let iterations = 4096;
//     let salt: Vec<u8> = (0..128)
//         .map(|_| random.gen_range(0, 2))
//         .collect();

//     pbkdf2(&mut Hmac::new(
//         Sha512::new(),
//         &entropy),
//         &salt,
//         iterations,
//         &mut buffer
//     );

//     println!("Seed: {:?}", buffer);

//     //Генерируем пару ключей
//     let (private_key, public_key) = ed25519::keypair(&buffer);
//     println!("Private key: {:?}\nPublic key: {:?}", private_key, public_key);

// }
