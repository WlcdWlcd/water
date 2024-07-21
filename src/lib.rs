pub mod args;
pub mod config;
pub mod init;
pub mod match_action;



pub fn help(){
    println!("

-h, -H                  for help
-d                      for drink default value
-D number               for drink entered value 
    example
    water -D 200 drink 200ml of water
    
    ")
}