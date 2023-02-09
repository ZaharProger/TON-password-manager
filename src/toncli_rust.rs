use std::{process::{Command, Output}, io::{Error, Write}, 
    str::from_utf8, fs::{File, read_to_string, remove_file}};

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
                // "/K lite-client -C global.config.json"
            ],
            RequestTypes::GenerateContractAddress => vec![
                "/K cd wallet",
                "func -o build\\contract.fif -SPA func\\stdlib.fc func\\code.func",
                "fift -s fift\\data_proxy.fif",
                "fift -s fift\\manipulation.fif build\\contract.fif build\\boc\\data.boc 0 build\\boc\\contract.boc build\\contract_address"
            ]
        };

        return Command::new(executor)
                .args(args.join(" && ").split(" "))
                .output()
                .expect(&self.command_fail_msg);
    }
    //Запись приватного ключа в файл
    fn save_key(&self, private_key: &[u8; 32]) {
        let file_name = "wallet\\build\\contract.pk";
        let mut file = File::create(file_name).unwrap();

        file.write_all(private_key).ok(); 
    }
    //Очищает все созданные сервисом файлы
    fn remove_builds(&self) {
        let files = vec![
            "wallet\\build\\boc\\data.boc",
            "wallet\\build\\boc\\contract.boc",
            "wallet\\build\\contract_address",
            "wallet\\build\\contract.addr",
            "wallet\\build\\contract.fif",
            "wallet\\build\\contract.pk"
        ];
        
        for file in files {
            remove_file(file).ok();
        }
    }
    //Получение адреса контракта
    pub fn get_contract_address(&self, private_key: &[u8; 32]) -> String {
        self.save_key(private_key);

        let mut items_counter = -1;
        let address_data = read_to_string("wallet\\build\\contract_address").unwrap();
        let contract_address = address_data
            .split(" ")
            .find(|_| {
                items_counter += 1;
                return items_counter == 1;
            });

        self.remove_builds();

        return contract_address.unwrap_or_else(|| "").to_string();
    }
    //Деплой контракта
    pub fn deploy_contract(&self, private_key: &[u8; 32]) -> ExecutionResult {
        self.save_key(private_key);
        self.execute_command(RequestTypes::GenerateContractAddress);
        let output = self.execute_command(RequestTypes::DeployContract);

        self.remove_builds();

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
    DeployContract,
    GenerateContractAddress
}

//Используется для возврата результата выполнения цепочки команд
pub struct ExecutionResult {
    pub result: bool,
    pub data: String,
    pub message: String
}