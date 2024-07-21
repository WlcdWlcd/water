use std::{env,process };
use water::args::parse_args::parse_args;

use water::match_action::match_action;
use water::init_config::init_config;

fn main() {
    init_config();
    let args:Vec<String> = env::args().collect();
    let parsed_args = parse_args(args).unwrap_or_else(|err| {
            println!("failed while parsing arguments: {err}");
            process::exit(-1);}
    );

    match_action(parsed_args);


    
}
