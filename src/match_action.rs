use crate::args::actions::ActionsEnum;
use crate::help;
use crate::config::check_config::check_config;


pub fn match_action(action: ActionsEnum){
    match action {
        ActionsEnum::Help => help(),
        ActionsEnum::CheckConfig=> check_config(),
        _=> {}
    }


}