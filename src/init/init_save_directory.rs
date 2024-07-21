use std::fs;
pub fn init_save_directory() -> std::io::Result<()>{
    fs::create_dir("/data")?;
    Ok(())
    
}