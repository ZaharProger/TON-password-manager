use std::any::Any;

//Используется для возврата результата выполнения цепочки команд
pub struct ExecutionResult {
    pub result: bool,
    pub data: String,
    pub message: String
}

//Трейт для структур аргументов
pub trait BaseArgs {
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