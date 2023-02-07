use std::{process::Command, fmt::Error,str};
pub struct ToncliRust {
    target_os: String,
}
impl ToncliRust {
    pub fn new(target_os: String) -> Result<ToncliRust, Error> {
        return Ok(ToncliRust { target_os });
    }

    pub fn test(&self) {
        let PATH:String = String::from("C:\\Users\\sheny\\seed\\TON-password-manager\\src\\wallet");

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args([PATH, "toncli".to_string()])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello Error")
                .output()
                .expect("failed to execute process")
        };
        let output: Vec<u8> = output.stdout;

        println!("{}", str::from_utf8(&output).unwrap()); 
    }
}
