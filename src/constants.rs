//Здесь будем перечислять все необходимые ОС для гибкой настройки Command
pub enum OStypes {
    Windows,
    Unix
}

//Здесь будем перечислять всевозможные действия с блокчейном
//без передачи доп аргументов
pub enum RequestTypes {
    GenerateContractAddress,
}

//Здесь будем перечислять всевозможные действия с блокчейном,
//которые требуют доп аргументы
pub enum RequestArgsTypes {
    DeployContract,
    SendTonsToContract
}