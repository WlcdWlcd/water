use std::fs::File;
use std::io::Write;
use std::io;

pub enum init_result{
    ConfigCreated,
    ConfigAlreadyExists

}


pub fn init_config() ->Result<init_result,io::Error> {
    let mut f = File::create_new("config.w");
    match f{

        Ok(mut file) => {
            file.write_all(
"defaultValue: 250"
            .as_bytes())?;
            Ok(init_result::ConfigCreated)
        }
        Err(err) => {
            match err.kind(){
                std::io::ErrorKind::AlreadyExists =>Ok(init_result::ConfigAlreadyExists),
                _ => {
                    println!("Error creating file: {}", err);
                    Err(err)
                
                }
            }
        }
    }
}