
use std::{ 
    fs::{metadata, File}, 
    io::Read, 
    sync::Arc, 
}; 
use native_api_1c::{
    native_api_1c_core::ffi::connection::Connection, 
    native_api_1c_macro::AddIn
}; 

#[derive(AddIn)] 
pub struct NativeApi { 
    #[add_in_con] // соединение с 1С для вызова внешних событий 
    connection: Arc<Option<&'static Connection>>, // Arc для возможности многопоточности 

    #[add_in_func(name = "ReadBytes", name_ru = "ПрочитатьБайты")] 
    #[arg(Str)] 
    #[returns(Blob, result)] 
    read_bytes: fn(&Self, String) -> Result<Vec<u8>, Box<dyn std::error::Error>>, 
} 

impl NativeApi { 
    pub fn new() -> Self { 
        Self { connection: Arc::new(None), 
        read_bytes: Self::read_bytes, } 
    } 

    pub fn read_bytes(&self, path: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> { 
        let mut f =  File::open(&path)?; 
        let metadata = metadata(&path)?; 
        let mut buffer = vec![0; metadata.len() as usize]; 
        f.read(&mut buffer)?; 
        Ok(buffer) 
    } 
}