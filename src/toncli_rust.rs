use std::{process::{Command, Output}, io::Error,
    str::from_utf8, fs::{write, read_to_string, remove_file}, env::current_dir};

pub struct ToncliRust {
    target_os: OStypes
}

impl ToncliRust {
    //Создание экземпляра структуры
    pub fn new(target_os: OStypes) -> Result<ToncliRust, Error> {
        return Ok(ToncliRust { 
            target_os
         });
    }

    //Формирование пути до файла в зависимости от ОС
    fn build_path(&self, parts: Vec<&str>, from_cwd: bool) -> String {
        let delimeter = if let OStypes::Windows = self.target_os { "\\" } else { "/" };
        let beginning = if let OStypes::Windows = self.target_os { 
            if from_cwd { "" } else { ".\\" }
        } 
        else { 
            if from_cwd { "/" } else { "" }
        };
        
        return format!("{}{}", beginning, parts.join(delimeter));
    }

    //Формирование команды и её исполнение
    fn execute_command(&self, request: RequestTypes) -> Output {
        let cwd = current_dir().unwrap().to_string_lossy().to_string();

        let executor = if let OStypes::Windows = self.target_os { "pwsh" } else { "sh" };
        let flag = if let OStypes::Windows = self.target_os { "-Command " } else { "" };
        let args = match request {
            RequestTypes::DeployContract => vec![
                format!("{}lite-client --timeout 10 -C global.config.json -c 'sendfile {}'", 
                    flag, self.build_path(vec![&cwd, "src", "wallet", "build", "boc", "contract.boc"], true))
            ],
            RequestTypes::GenerateContractAddress => vec![
                format!("{}cd {}", flag, self.build_path(vec!["src", "wallet"], false)),
                format!("func -o {} -SPA {} {}", 
                    self.build_path(vec!["build", "contract.fif"], false), 
                    self.build_path(vec!["func", "stdlib.fc"], false), 
                    self.build_path(vec!["func", "code.func"], false)
                ),
                format!("fift -s {}", self.build_path(vec!["fift", "data_proxy.fif"], false)),
                format!("fift -s {} {} {} 0 {} {}", 
                    self.build_path(vec!["fift", "manipulation.fif"], false),
                    self.build_path(vec!["build", "contract.fif"], false), 
                    self.build_path(vec!["build", "boc", "data.boc"], false),
                    self.build_path(vec!["build", "boc", "contract.boc"], false), 
                    self.build_path(vec!["build", "contract_address"], false)
                )
            ]
        };

        let handler = Command::new(executor)
                .args(args.join(" && ").split(" "))
                .spawn()
                .expect("Ошибка выполнения команды");
        
        return handler.wait_with_output().unwrap();
    }

    //Запись приватного ключа в файл
    fn save_key(&self, private_key: &[u8; 32]) { 
        let path = self.build_path(
            vec!["src", "wallet", "build", "contract.pk"], false);
        write(path, *private_key).ok();
    }

    //Очищает все созданные сервисом файлы
    pub fn finish(&self) {
        let files = vec![
            self.build_path(vec!["boc", "data.boc"], false),
            self.build_path(vec!["boc", "contract.boc"], false),
            self.build_path(vec!["contract.fif"], false),
            self.build_path(vec!["contract_address"], false),
            self.build_path(vec!["contract.addr"], false),
            self.build_path(vec!["contract.pk"], false),
        ];
        
        for file in files {
            remove_file(self.build_path(
                vec!["src", "wallet", "build", &file], false)).ok();
        }
    }

    //Получение адреса контракта
    pub fn get_contract_address(&self, private_key: &[u8; 32]) -> String {
        self.save_key(private_key);

        let path = self.build_path(
            vec!["src", "wallet", "build", "contract_address"], false);

        let mut items_counter = -1;
        let address_data = read_to_string(path).unwrap();
        let contract_address = address_data
            .split(" ")
            .find(|_| {
                items_counter += 1;
                return items_counter == 1;
            });

        return contract_address.unwrap_or_else(|| "").to_string();
    }

    //Деплой контракта
    pub fn deploy_contract(&self, private_key: &[u8; 32]) -> ExecutionResult {
        self.save_key(private_key);

        self.execute_command(RequestTypes::GenerateContractAddress);
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
    Unix
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