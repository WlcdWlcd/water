use std::fs;

pub fn check_config(){
    let config = fs::read_to_string("config.w").expect(
        "error reading config"
    );
    println!("
your configuration is:    
    {config}");

}