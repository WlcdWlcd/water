use std::io;

use std::io::ErrorKind::InvalidInput;

use crate::args::actions::ActionsEnum;

pub fn parse_args(args: Vec<String>)->Result<ActionsEnum,io::Error>{
    let mut iterator = args.iter();
    iterator.next();
    
    if let Some(action) = iterator.next() {
        
        match action.as_str() {
            "-d" =>Ok(ActionsEnum::DrinkDefault),
            "-D"=>{
                if let Some(count)= iterator.next(){
                    if let Ok(count_i) = count.parse::<usize>(){
                        Ok(ActionsEnum::DrinkCustom(count_i))
                    }
                    else{
                        Err(io::Error::new(InvalidInput,"Invalid drink count provided, drink count must be a positive number"))
                    }
                }
                else{
                    Err(io::Error::new(InvalidInput,"Drink count not provided"))
                }


            }
            "-h" | "-H"=>Ok(ActionsEnum::Help),
            "-c" =>Ok(ActionsEnum::CheckConfig),
            "-C"=>Ok(ActionsEnum::SetConfig),
            _=>{
                
                Err(io::Error::new(InvalidInput,"Ivalid argumets to help run with -h"))}
        }
    }
    else{
        Ok(ActionsEnum::Stats) 
    }




}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d(){
        let args = vec!["E:\\folder\\water.exe".to_string(),
        "-d".to_string()];
        assert_eq!(parse_args(args).unwrap(), ActionsEnum::DrinkDefault);

    }

    #[test]
    fn test_D_with_entered_value(){
        let args = vec!["E:\\folder\\water.exe".to_string(),
        "-D".to_string(),
         "200".to_string()];
        assert_eq!(parse_args(args).unwrap(), ActionsEnum::DrinkCustom(200));
    }
    #[test]
    fn test_D_with_no_entered_value(){
        let args = vec!["E:\\folder\\water.exe".to_string(),
        "-D".to_string()];
        
        
    }

    #[test]
    fn test_empty(){
        let args = vec!["E:\\folder\\water.exe".to_string()];
        assert_eq!(parse_args(args).unwrap(), ActionsEnum::Stats);
    }
}