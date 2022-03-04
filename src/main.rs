// standard imports
use std::net;

//                  File IO imports
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Dependencies imports
use console::style;


fn main() {
    log_info("Starting Program", "ヾ(•ω•`)o");
    log_info("Test Saving", "[]~(￣▽￣)~*");
    let mut i : u32 = 0;
    let submission : &str;
    while i < 50 {
        i.to_string();
        save_to_file("1");
        i = i + 1;
    }
}

// Logging Functions

fn log_info(msg : &str, emoji : &str){
    println!("{} ... {}" , msg ,  style(emoji).green());
}

fn log_error(msg : &str, emoji : &str){
    println!("{} ... {}" , msg ,  style(emoji).red());
}

fn start_server(){

}

fn save_to_file(data: &str){
    let file_path = Path::new("instructions.que");
    
    let mut file = match File::create(&file_path) {
        Err(e) => panic!("couldnt open file : {} ... {}", e, style("OwO").red()),
        Ok(file) => file,
    };
    file.write(data.as_bytes());
    
    log_info("File Saved", "(ﾉ*･ω･)ﾉ")
    
}

fn append_to_file(data : String) {
    let file_path = Path::new("instructions.que");
    
    let mut file = match File::create(&file_path) {
        Err(e) => panic!("couldnt open file : {} ... {}", e, style("OwO").red()),
        Ok(file) => file,
    };
    
    file.write(data<u8>);
    
    log_info("File Saved", "(ﾉ*･ω･)ﾉ")
    
}