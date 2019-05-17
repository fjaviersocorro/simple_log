extern crate chrono;

use self::chrono::Local;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::fmt::{Arguments};

//Because of the us of macros we need to write #[macro_use] whenever we want to use them

#[macro_export]
macro_rules! t_event {
    ($($arg:tt)*) => ($crate::log::write_event(format_args!($($arg)*), "[INFO]      "));
}

#[macro_export]
macro_rules! w_event {
    ($($arg:tt)*) => ($crate::log::write_event(format_args!($($arg)*), "[WARNING]   "));
}
#[macro_export]
macro_rules! e_event {
    ($($arg:tt)*) => ($crate::log::write_event(format_args!($($arg)*), "[ERROR]     "));
}


pub fn write_event(args: Arguments, kind: &str){

    let date = Local::now();
    let d = date.format("[%Y-%m-%d] [%H:%M:%S]: ").to_string();

    let x = kind;
    let message = format!("{:?}", args);
    let message = d +  &x + &message  + "\n";

    if  Path::new("log.log").exists(){
        let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("log.log")
        .unwrap();

        if let Err(e) = write!(file, "{}", message) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }else{
        let path = Path::new("log.log");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
            Ok(file) => file,
        };

        
        match file.write_all(message.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                               why.description())
            },
            Ok(_) => println!("Log file created at {}", display),
        }
    }

    
}