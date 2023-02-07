use std::{process::{Command, Output}, io::{Error, Read}, str::from_utf8};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;

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
            RequestTypes::DeployContract => "/K cd wallet && toncli -h"
        };

        return Command::new(executor)
                .args(args.split(" "))
                .output()
                .expect(&self.command_fail_msg);
    }
    //Деплой контракта
    pub fn deploy_contract(&self) -> ExecutionResult {
        let output = self.execute_command(RequestTypes::DeployContract);

        //Эта штука вроде конвертит Unicode, но получается хрень конкретная
        // let encoder = Encoding::for_label("Latin1".as_bytes());
        // let mut decoder = DecodeReaderBytesBuilder::new()
        //     .encoding(encoder)
        //     .build(if let 0 = output.stderr.len() { 
        //         &output.stdout[..]
        //      } 
        //      else { 
        //         &output.stderr[..] 
        //     }); 
                    
        // let mut encoded_data = String::new();
        // decoder.read_to_string(&mut encoded_data).unwrap();

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