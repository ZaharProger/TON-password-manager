pub mod seed_phrase;
pub mod toncli_rust;
pub mod entities;
pub mod constants;

use std::time::Duration;
use async_std::task::{spawn, sleep};

fn main() {
    let toncli = toncli_rust::ToncliRust::new(
        constants::OStypes::Windows
    ).unwrap();

    let mut seed_phrase = seed_phrase::SeedPhrase::new(4096).unwrap();
    let private_key = seed_phrase.get_private_key();
    toncli.save_key(&private_key);

    let (bounceable_address, non_bounceable_address) = toncli.get_contract_addresses();
    println!("{} {}", bounceable_address, non_bounceable_address);

    toncli.send_tons_to_wallet(&entities::SendTonsArgs {
        address: non_bounceable_address,
        subwallet_id: 0,
        seqno: 9,
        tons_amount: 0.05
    });

    spawn(async move {
        sleep(Duration::from_secs(10)).await;
        
        let result = toncli.deploy_contract();
        
        println!("{}", result.result);
        println!("{}", result.data);
        println!("{}", result.message);

        toncli.finish();
    });
}
