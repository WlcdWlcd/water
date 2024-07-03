use std::env;
use water::args::parse_args::parse_args;

fn main() {
    let args:Vec<String> = env::args().collect();
    dbg!(parse_args(args));


    
}
