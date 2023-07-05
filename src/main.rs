#![allow(dead_code, unused_variables)]

use std::{io, env};
use tokio;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum Command{
    Exit,
    ReadFromFile,
    Sleep,
}

impl Command {
    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
            "exit" => Ok(Command::Exit),
            "readfromfile" => Ok(Command::ReadFromFile),
            "sleep" => Ok(Command::Sleep),
            _ => Err(()),
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input = "".to_string();
        stdin.read_line(&mut input).unwrap();
        if input.ends_with('\n') {
            input.truncate(input.len()-1);
        }
        if let Ok(command) = Command::from_str(&input) {
            println!("input {:?} ", command);
            match command {
                Command::Exit => 
                    break,
                Command::ReadFromFile => 
                    tokio::spawn(read_from_file("./file.txt".into())),
                Command::Sleep =>
                    tokio::spawn(sleeper(5, input)),
            };
        } else {
            println!("Invalid input.");
        };
    }

}

async fn sleeper(secs: u64, name: String) {
    println!("{} sleeper started with {} secs", name, secs);
    sleep(Duration::from_secs(secs)).await;
    println!("{} slept {} secs", name, secs);
}

async fn read_from_file(path: String) {
    println!("Read file {}", path);
    let read_result = tokio::fs::read_to_string(path.clone()).await;
    match read_result {
        Ok(file_content) => {
            println!("Reading file {} done. Content: {}", path, file_content);
        }
        Err(e) => {
            println!("An error occured during reading file {}. Error: {}", path, e);
        }
    }
}