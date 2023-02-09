use std::{process::{Command, Child}, io::Error,
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
    fn build_path(&self, parts: Vec<&str>) -> String {
        return parts.join(if let OStypes::Windows = self.target_os { "\\" } else { "/" });
    }

    //Формирование команды и её исполнение
    fn execute_command(&self, request: RequestTypes) -> Child {
        let cwd = current_dir().unwrap().to_string_lossy().to_string();

        let executor = if let OStypes::Windows = self.target_os { "cmd" } else { "sh" };
        let flag = if let OStypes::Windows = self.target_os { "/K " } else { "" };
        let args = match request {
            RequestTypes::DeployContract => vec![
                format!("{}lite-client --timeout 10 -C global.config.json -c 'sendfile {}'",
                flag, self.build_path(vec![&cwd, "src", "wallet", "build", "boc", "contract.boc"]))
            ],
            RequestTypes::GenerateContractAddress => vec![
                format!("{}cd {}", flag, self.build_path(vec!["src", "wallet"])),
                format!("func -o {} -SPA {} {}", 
                    self.build_path(vec!["build", "contract.fif"]), 
                    self.build_path(vec!["func", "stdlib.fc"]), 
                    self.build_path(vec!["func", "code.func"])
                ),
                format!("fift -s {}", self.build_path(vec!["fift", "data_proxy.fif"])),
                format!("fift -s {} {} {} 0 {} {}", 
                    self.build_path(vec!["fift", "manipulation.fif"]),
                    self.build_path(vec!["build", "contract.fif"]), 
                    self.build_path(vec!["build", "boc", "data.boc"]),
                    self.build_path(vec!["build", "boc", "contract.boc"]), 
                    self.build_path(vec!["build", "contract_address"])
                )
            ]
        };

        return Command::new(executor)
                .args(args.join(" && ").split(" "))
                .spawn()
                .expect("Ошибка выполнения команды");
    }

    //Запись приватного ключа в файл
    fn save_key(&self, private_key: &[u8; 32]) { 
        let path = self.build_path(vec!["src", "wallet", "build", "contract.pk"]);
        write(path, *private_key).ok();
    }

    //Очищает все созданные сервисом файлы
    fn remove_builds(&self) {
        let files = vec![
            self.build_path(vec!["boc", "data.boc"]),
            self.build_path(vec!["boc", "contract.boc"]),
            self.build_path(vec!["contract.fif"]),
            self.build_path(vec!["contract_address"]),
            self.build_path(vec!["contract.addr"]),
            self.build_path(vec!["contract.pk"]),
        ];
        
        for file in files {
            remove_file(self.build_path(vec!["src", "wallet", "build", &file])).ok();
        }
    }

    //Получение адреса контракта
    pub fn get_contract_address(&self, private_key: &[u8; 32]) -> String {
        self.save_key(private_key);

        let path = self.build_path(vec!["src", "wallet", "build", "contract_address"]);

        let mut items_counter = -1;
        let address_data = read_to_string(path).unwrap();
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
        let output = self.execute_command(RequestTypes::DeployContract)
            .wait_with_output()
            .unwrap();

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