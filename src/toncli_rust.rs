use std::{process::{Command, Output}, io::{Error, Write}, str::from_utf8, fs::File};

pub struct ToncliRust {
    target_os: OStypes,
    command_fail_msg: String
}

impl ToncliRust {
    //Создание экземпляра структуры
    pub fn new(target_os: OStypes) -> Result<ToncliRust, Error> {
        return Ok(ToncliRust { 
            target_os,
            command_fail_msg: "Ошибка выполнения команды".to_string()
         });
    }
    //Формирование команды и её исполнение
    fn execute_command(&self, request: RequestTypes) -> Output {
        let executor = if let OStypes::Windows = self.target_os { "cmd" } else { "sh" };
        let args = match request {
            RequestTypes::DeployContract => vec![
                "/K cd wallet",
                "func -o build\\contract.fif -SPA C:\\TON\\smartcont\\stdlib.fc func\\code.func",
                "fift -s fift\\data_proxy.fif",
                "fift -s fift\\manipulation.fif build\\contract.fif build\\boc\\data.boc 0 build\\boc\\contract.boc build\\contract_address",
                // "/K lite-client -C C:\\TON\\global.config.json"
            ]
        };

        return Command::new(executor)
                .args(args.join(" && ").split(" "))
                .output()
                .expect(&self.command_fail_msg);
    }
    //Деплой контракта
    pub fn deploy_contract(&self, private_key: &[u8; 32]) -> ExecutionResult {
        let file_name = "wallet\\build\\contract.pk";
        let mut file = File::create(file_name).unwrap();
        file.write_all(private_key).ok(); 
        
        let output = self.execute_command(RequestTypes::DeployContract);

        return if let 0 = output.stderr.len() {
            ExecutionResult {
                result: true,
                data: from_utf8(&output.stdout).unwrap().to_string(),
                message: "".to_string()
            }
        }
        else {
            ExecutionResult {
                result: false,
                data: "".to_string(),
                message: from_utf8(&output.stderr).unwrap().to_string()
            }
        };
    }
}

//Здесь будем перечислять все необходимые ОС для гибкой настройки Command
pub enum OStypes {
    Windows,
    Linux
}

//Здесь будем перечислять всевозможные действия с блокчейном
//для определения набора аргументов командной строки
enum RequestTypes {
    DeployContract
}

//Используется для возврата результата выполнения цепочки команд
pub struct ExecutionResult {
    pub result: bool,
    pub data: String,
    pub message: String
}