use crate::entities::{BaseArgs, DeployContractArgs, SendTonsArgs, ExecutionResult};
use crate::constants::{OStypes, RequestArgsTypes, RequestTypes};

use std::{process::{Command, Output}, io::Error,
    str::from_utf8, fs::{write, read_to_string, remove_file}, env::current_dir};

pub struct ToncliRust {
    target_os: OStypes,
    flag: String,
    executor: String
}

impl ToncliRust {
    //Создание экземпляра структуры
    pub fn new(target_os: OStypes) -> Result<ToncliRust, Error> {
        let executor = if let OStypes::Windows = target_os 
            { "powershell".to_string() } else { "sh".to_string() };

        let flag = if let OStypes::Windows = target_os 
            { "-Command ".to_string() }  else { "".to_string() };

        return Ok(ToncliRust { 
            target_os,
            flag,
            executor
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
        let args = match request {
            RequestTypes::GenerateContractAddress => vec![
                format!("{}cd {}", self.flag, self.build_path(vec!["src", "wallet"], false)),
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

        let handler = Command::new(&self.executor)
                .args(args.join(" ; ").split(" "))
                .spawn()
                .expect("Ошибка выполнения команды");
        
        return handler.wait_with_output().unwrap();
    }

    //Формирование команды с аргументами и её исполнение
    fn execute_command_with_args(&self, request: RequestArgsTypes, args_values: &dyn BaseArgs) -> Output {
        let args = match request {
            RequestArgsTypes::DeployContract => {
                let values = args_values.as_any()
                    .downcast_ref::<DeployContractArgs>()
                    .unwrap();

                vec![format!("{}lite-client --timeout 10 -C global.config.json -c 'sendfile {}'", 
                    self.flag, 
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
                        self.flag, 
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

        let handler = Command::new(&self.executor)
                .args(args.join(" ; ").split(" "))
                .spawn()
                .expect("Ошибка выполнения команды");
        
        return handler.wait_with_output().unwrap();
    }

    //Запись приватного ключа в файл
    pub fn save_key(&self, private_key: &[u8; 32]) { 
        let path = self.build_path(
            vec!["src", "wallet", "build", "contract.pk"], false);
        write(path, *private_key).ok();

        self.execute_command(RequestTypes::GenerateContractAddress);
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
    pub fn get_contract_addresses(&self) -> (String, String) {
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
    pub fn send_tons_to_wallet(&self, args: &SendTonsArgs) -> ExecutionResult {
        let output = self.execute_command_with_args(
            RequestArgsTypes::SendTonsToContract, args);

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
    pub fn deploy_contract(&self) -> ExecutionResult {
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