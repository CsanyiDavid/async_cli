use std::{io, path::PathBuf, str::FromStr};
use tokio;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum Command {
    Exit,
    Sleep(u64),
    ReadFromFile(PathBuf),
    CreateFile(PathBuf),
    CreateDir(PathBuf),
}

#[derive(Debug)]
struct ParseError();

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_of_s: Vec<&str> = s.split(' ').collect();
        match parts_of_s[0] {
            "exit" => Ok(Command::Exit),
            "sleep" => {
                if parts_of_s.len() != 2 {
                    return Err(ParseError())
                }
                let seconds_result = parts_of_s[1].parse::<u64>();
                match seconds_result {
                    Ok(seconds) => Ok(Command::Sleep(seconds)),
                    Err(_) => Err(ParseError()),
                }
            }
            "readfromfile" => {
                if parts_of_s.len() != 2 {
                    return Err(ParseError())
                }
                let path = PathBuf::from(parts_of_s[1]);
                Ok(Command::ReadFromFile(path))
            }
            "createfile" => {
                if parts_of_s.len() != 2 {
                    return Err(ParseError())
                }
                let path = PathBuf::from(parts_of_s[1]);
                Ok(Command::CreateFile(path))
            }
            "createdir" => {
                if parts_of_s.len() != 2 {
                    return Err(ParseError())
                }
                let path = PathBuf::from(parts_of_s[1]);
                Ok(Command::CreateDir(path))
            },
            _ => Err(ParseError()),
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
        if let Ok(command) = input.parse() {
            println!("input {:?} ", command);
            match command {
                Command::Exit => 
                    break,
                Command::Sleep(seconds) =>
                    tokio::spawn(sleeper(seconds)),
                Command::ReadFromFile(path) => 
                    tokio::spawn(read_from_file(path)),
                Command::CreateFile(path) =>
                    tokio::spawn(create_file(path)),
                Command::CreateDir(path) =>
                    tokio::spawn(create_dir(path)),
            };
        } else {
            println!("Invalid input.");
        };
    }

}

async fn sleeper(seconds: u64) {
    println!("sleeper started with {} seconds", seconds);
    sleep(Duration::from_secs(seconds)).await;
    println!("slept {} seconds", seconds);
}

async fn read_from_file(path: PathBuf) {
    println!("Read file {:?}", path);
    let read_result
        = tokio::fs::read_to_string(path.clone()).await;
    match read_result {
        Ok(file_content) => {
            println!("Reading file {:?} done. Content: {}", path, file_content);
        }
        Err(e) => {
            println!("An error occured during reading file {:?}. Error: {}", path, e);
        }
    };
}

async fn create_file(path: PathBuf) {
    println!("Create file {:?}", path);
    match  tokio::fs::try_exists(path.clone()).await {
        Ok(true) => {},
        _ => {
            let parent_dir = path.parent().unwrap();
            create_dir(PathBuf::from(parent_dir)).await;
        },
    };
    let create_result = tokio::fs::File::create(path.clone()).await;
    match create_result {
        Ok(_) => println!("File created {:?}", path),
        Err(e) => println!("An error occured during creating file {:?}. Error: {}", path, e),
    };
}

async fn create_dir(path: PathBuf) {
    println!("Create dir {:?}", path);
    let create_result
        = tokio::fs::create_dir_all(path.clone()).await;
    match create_result {
        Ok(_) => println!("Dir created {:?}", path),
        Err(e) => println!("An error occured during creating dir {:?}. Error: {}", path, e),
    };
}
