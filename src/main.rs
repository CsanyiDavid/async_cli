use std::{path::PathBuf, str, str::FromStr};
use tokio;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum Command {
    Exit,
    Sleep(u64),
    Count(u64),
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
            "count" => {
                if parts_of_s.len() != 2 {
                    return Err(ParseError())
                }
                let cnt_result = parts_of_s[1].parse::<u64>();
                match cnt_result {
                    Ok(cnt) => Ok(Command::Count(cnt)),
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut task_handles = Vec::<tokio::task::JoinHandle<()>>::new();
    loop {
        let input = read().await;
        /*if input.ends_with('\n') {
            input.truncate(input.len()-1);
        }*/
        if let Ok(command) = input.parse() {
            println!("Input command: {:?} ", command);
            let handle = match command {
                Command::Exit => 
                    break,
                Command::Sleep(seconds) =>
                    tokio::spawn(sleeper(seconds)),
                Command::Count(cnt) =>
                    tokio::spawn(counter(cnt)),
                Command::ReadFromFile(path) => 
                    tokio::spawn(read_from_file(path)),
                Command::CreateFile(path) =>
                    tokio::spawn(create_file(path)),
                Command::CreateDir(path) =>
                    tokio::spawn(create_dir(path)),
            };
            task_handles.push(handle);
        } else {
            println!("Invalid input.");
        };
    }
    println!("Waiting for running tasks...");
    for handle in task_handles {
        match handle.await {
            Ok(_) => {},
            Err(e) => println!("Error {e}")
        };
    }
    println!("All tasks done. Exit");

}

async fn read() -> String {
    println!("Reading started..");
    let mut stdin = tokio::io::stdin();
    let mut buffer = [0;50];
    let n = stdin.read(&mut buffer[..]).await.unwrap();
    let s = match str::from_utf8(&buffer[..n-1]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("Read {}.", s);
    s.into()
}

async fn sleeper(seconds: u64) {
    println!("Sleeper started with {} seconds.", seconds);
    sleep(Duration::from_secs(seconds)).await;
    println!("Slept {} seconds.", seconds);
}

async fn counter(cnt: u64) {
    for i in 0..cnt+1 {
        println!("count {}", i);
        sleep(Duration::from_secs(1)).await;
    }
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
