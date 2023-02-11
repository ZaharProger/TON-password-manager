use std::{process::{Command, Output}, io::Error,
    str::from_utf8, fs::{write, read_to_string, remove_file}, env::current_dir, any::Any};

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

    //Формирование команды без аргументов и её исполнение
    fn execute_command(&self, request: RequestTypes) -> Output {
        let executor = if let OStypes::Windows = self.target_os { "powershell" } else { "sh" };
        let flag = if let OStypes::Windows = self.target_os { "-Command " } else { "" };

        let args = match request {
            RequestTypes::GenerateContractAddress => vec![
                format!("{}cd {}", flag, self.build_path(vec!["src", "wallet"], false)),
                format!("func -o {} -SPA {} {} {} {}", 
                    self.build_path(vec!["build", "contract.fif"], false), 
                    self.build_path(vec!["func", "stdlib.fc"], false), 
                    self.build_path(vec!["func", "error_codes.func"], false), 
                    self.build_path(vec!["func", "math.func"], false), 
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
                .args(args.join(" ; ").split(" "))
                .spawn()
                .expect("Ошибка выполнения команды");
        
        return handler.wait_with_output().unwrap();
    }

    //Формирование команды с аргументами и её исполнение
    fn execute_command_with_args(&self, request: RequestArgsTypes, args_values: &dyn BaseArgs) -> Output {
        let executor = if let OStypes::Windows = self.target_os { "powershell" } else { "sh" };
        let flag = if let OStypes::Windows = self.target_os { "-Command " } else { "" };

        let args = match request {
            RequestArgsTypes::DeployContract => {
                let values = args_values.as_any()
                    .downcast_ref::<DeployContractArgs>()
                    .unwrap();

                vec![format!("{}lite-client --timeout 10 -C global.config.json -c 'sendfile {}'", 
                    flag, 
                    self.build_path(
                        vec![&values.cwd, "src", "wallet", "build", "boc", "contract.boc"], 
                        true
                    )
                )]
            },
            RequestArgsTypes::SendTonsToContract => {
                let values = args_values.as_any()
                    .downcast_ref::<SendTonsArgs>()
                    .unwrap();

                vec![format!("{}fift -s {} {} {} {} {} {}", 
                        flag, 
                        self.build_path(vec!["src", "wallet", "fift", "usage.fif"], false),
                        self.build_path(vec!["src", "deploy_wallet", "contract"], false),
                        values.address,
                        values.subwallet_id,
                        values.seqno,
                        values.tons_amount),
                    format!("lite-client --timeout 10 -C global.config.json -c 'sendfile {}'",
                        self.build_path(vec!["src", "wallet", "build", "boc", "usage.boc"], true)    
                    )
                ]
            }
        };

        let handler = Command::new(executor)
                .args(args.join(" ; ").split(" "))
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
    pub fn get_contract_addresses(&self, private_key: &[u8; 32]) -> (String, String) {
        self.save_key(private_key);

        let path = self.build_path(
            vec!["src", "wallet", "build", "contract_address"], false);
        let address_data = read_to_string(path).unwrap();

        let mut items_counter = -1;
        let bounceable_address = address_data
            .split(" ")
            .find(|_| {
                items_counter += 1;
                return items_counter == 1;
            });
        
        items_counter = -1;
        let non_bounceable_address = address_data
            .split(" ")
            .find(|_| {
                items_counter += 1;
                return items_counter == 2;
            });

        return (bounceable_address.unwrap().to_string(), 
            non_bounceable_address.unwrap().to_string());
    }

    //Отправка тонов через деплой кошелек
    pub fn send_tons_to_wallet(&self, private_key: &[u8; 32]) -> ExecutionResult {
        self.save_key(private_key);

        self.execute_command(RequestTypes::GenerateContractAddress);
        let (_, address) = self.get_contract_addresses(private_key);
        let output = self.execute_command_with_args(
            RequestArgsTypes::SendTonsToContract, &SendTonsArgs {
                address,
                subwallet_id: 0,
                seqno: 7,
                tons_amount: 0.05
            }
        );

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

    //Деплой контракта
    pub fn deploy_contract(&self, private_key: &[u8; 32]) -> ExecutionResult {
        self.save_key(private_key);

        let output = self.execute_command_with_args(
            RequestArgsTypes::DeployContract, 
            &DeployContractArgs {
            cwd: current_dir().unwrap().to_string_lossy().to_string()
            }
        );

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
//без передачи доп аргументов
enum RequestTypes {
    GenerateContractAddress,
}

//Здесь будем перечислять всевозможные действия с блокчейном,
//которые требуют доп аргументы
enum RequestArgsTypes {
    DeployContract,
    SendTonsToContract
}

//Используется для возврата результата выполнения цепочки команд
pub struct ExecutionResult {
    pub result: bool,
    pub data: String,
    pub message: String
}

//Трейт для структур аргументов
trait BaseArgs {
    fn as_any(&self) -> &dyn Any;
}

//Аргументы для отправки тонов
pub struct SendTonsArgs {
    pub address: String,
    pub subwallet_id: u64,
    pub seqno: u64,
    pub tons_amount: f64
}

//Аргументы для деплоя контракта
pub struct DeployContractArgs {
    pub cwd: String
}

//Реализация трейта для всех структур аргументов
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl BaseArgs for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }
        })*
    }
}

impl_T!(for SendTonsArgs, DeployContractArgs);