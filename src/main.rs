#![allow(dead_code, unused_variables)]

use std::{io, env};
use tokio;
use tokio::time::{sleep, Duration};

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
        println!("input {} ", input);
        tokio::spawn(sleeper(5, input));
        tokio::spawn(read_from_file("./file.txt".into()));
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