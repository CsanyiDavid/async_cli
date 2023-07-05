use std::{io, thread::spawn};
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
    }

}

async fn sleeper(secs: u64, name: String) {
    println!("{} sleeper started with {} secs", name, secs);
    sleep(Duration::from_secs(secs)).await;
    println!("{} slept {} secs", name, secs);
}
