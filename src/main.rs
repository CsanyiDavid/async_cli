#![allow(dead_code, unused_variables)]

use std::{io, env, future::Future};
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
                    tokio::spawn(read_from_file("./base/file.txt".into())),
                Command::Sleep =>
                    tokio::spawn(sleeper(5, input)),
                Command::CreateFile =>
                    tokio::spawn(create_file("./base/sub/created.txt".into())),
                Command::CreateDir =>
                    tokio::spawn(create_dir("./base/sub".into())),
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
    };
}

async fn create_file(path: String) {
    println!("Create file {}", path);
    let create_result = tokio::fs::File::create(path.clone()).await;
    match create_result {
        Ok(file) => println!("File created {}", path),
        Err(e) => println!("An error occured during creating file {}. Error: {}", path, e),
    };
}

fn create_dir(path: String) -> impl Future<Output = ()> {
    async move {
        println!("Create dir {}", path);
        let create_result = tokio::fs::create_dir(path.clone()).await;
        match create_result {
            Ok(file) => println!("Dir created {}", path),
            Err(e) => println!("An error occured during creating dir {}. Error: {}", path, e),
        };
    }
}

async fn wait_for_path_to_exist(path: String) {

}