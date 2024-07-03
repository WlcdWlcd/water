use std::{env,process };
use water::args::parse_args::parse_args;
use water::args::actions::ActionsEnum;

use water::help::help;

fn main() {
    let args:Vec<String> = env::args().collect();
    let parsed_args = parse_args(args).unwrap_or_else(|err| {
            println!("failed while parsing arguments: {err}");
            process::exit(-1);}
    );

    match parsed_args{
        ActionsEnum::Help => help(),

        _=> {}
    }


    
}
