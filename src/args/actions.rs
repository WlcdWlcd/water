#[derive(PartialEq, Debug)]
pub enum ActionsEnum {
    DrinkDefault,
    DrinkCustom(usize),
    Stats,
    Help,
    SetConfig,
    CheckConfig
    

}
