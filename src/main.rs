use std::env;
use std::error::Error;

type CommandResult<E = Box<dyn Error>> = Result<i32, E>;

fn help() -> CommandResult {
    println!("help");
    Ok(0)
}

fn play(_args: Vec<String>) -> CommandResult {
    println!("play wordle!");
    Ok(0)
}

fn solve(_args: Vec<String>) -> CommandResult {
    println!("solve wordle!");
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1).map(|s| s.trim().to_string());
    match args.next().as_deref() {
        Some("play") => play(args.map(String::from).collect()),
        Some("solve") => solve(args.map(String::from).collect()),
        Some("help") | Some("-h") | Some(_) | None => help(),
    }.and(Ok(()))
}
