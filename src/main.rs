// standard imports
use std::net;

//                  File IO imports
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;


// Dependencies imports
use console::style;


fn main() {
    log_info("Starting Program", "ヾ(•ω•`)o");
    log_info("Test Saving", "[]~(￣▽￣)~*");
    let mut i : u8 = 0;
    let mut submission :String;
    save_to_file(String::from(" "));
    while i < 255 {
        submission = i.to_string() + "\n";
        append_to_file(submission);
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

// FILE I/O Functions

    fn save_to_file(data: String){
        let file_path = Path::new("instructions.que");
        
        let mut file = match File::create(&file_path) {
            Err(e) => panic!("couldnt open file : {} ... {}", e, style("OwO").red()),
            Ok(file) => file,
        };
        file.write(&data.as_bytes()).expect("error");
        
        log_info("File Saved", "(ﾉ*･ω･)ﾉ")
        
    }

    fn append_to_file(data : String) {
        let file_path = Path::new("instructions.que");
        
        let mut file = OpenOptions::new().append(true).open(file_path).expect("cannot open file");
        
        file.write(&data.as_bytes()).expect("error");

        log_info("Added item to queue", "(ﾉ*･ω･)ﾉ")
        
    }