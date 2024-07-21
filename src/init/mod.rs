pub mod init_config;
pub mod init_save_directory;


pub fn init(){
    init_config::init_config();
    init_save_directory::init_save_directory();
}