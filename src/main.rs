#![allow(dead_code, unused_variables, unused_imports)]

use core::panic;
use std::fmt::write;
use std::{io, future::Future, path::Path, fmt};
use tokio;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum Command{
    Exit,
    Sleep,
    ReadFromFile,
    CreateFile,
    CreateDir,
}

impl Command {
    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
            "exit" => Ok(Command::Exit),
            "readfromfile" => Ok(Command::ReadFromFile),
            "sleep" => Ok(Command::Sleep),
            "createfile" => Ok(Command::CreateFile),
            "createdir" => Ok(Command::CreateDir),
            _ => Err(()),
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut input;
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
                    tokio::spawn(read_from_file(
                        Path::new("./base/file.txt")
                    )),
                Command::Sleep =>
                    tokio::spawn(sleeper(5, input)),
                Command::CreateFile =>
                    tokio::spawn(create_file(
                        Path::new("./base/sub/created.txt")
                    )),
                Command::CreateDir =>
                    tokio::spawn(create_dir(
                        Path::new("./base/sub")
                    )),
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

async fn read_from_file(path: &Path) {
    println!("Read file {:?}", path);
    let read_result
        = tokio::fs::read_to_string(path).await;
    match read_result {
        Ok(file_content) => {
            println!("Reading file {:?} done. Content: {}", path, file_content);
        }
        Err(e) => {
            println!("An error occured during reading file {:?}. Error: {}", path, e);
        }
    };
}

async fn create_file(path: &Path) {
    println!("Create file {:?}", path);
    match  tokio::fs::try_exists(path).await {
        Ok(true) => {},
        _ => {
            let parent_dir = path.parent().unwrap();
            create_dir(parent_dir).await;
        },
    };
    let create_result = tokio::fs::File::create(path).await;
    match create_result {
        Ok(file) => println!("File created {:?}", path),
        Err(e) => println!("An error occured during creating file {:?}. Error: {}", path, e),
    };
}

async fn create_dir(path: &Path) {
    println!("Create dir {:?}", path);
    let create_result
        = tokio::fs::create_dir_all(path).await;
    match create_result {
        Ok(file) => println!("Dir created {:?}", path),
        Err(e) => println!("An error occured during creating dir {:?}. Error: {}", path, e),
    };
}
