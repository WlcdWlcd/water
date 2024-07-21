#[derive(PartialEq, Debug)]
pub enum ActionsEnum {
    DrinkDefault,
    DrinkEnter(usize),
    Stats,
    Help,
    SetConfig,
    CheckConfig
    

}
