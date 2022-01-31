use std::env;
use std::error::Error;

type CommandResult<E = Box<dyn Error>> = Result<(), E>;

fn help() -> CommandResult {
    println!("help");
    Ok(())
}

fn play(_args: Vec<String>) -> CommandResult {
    println!("play wordle!");
    Ok(())
}

fn solve(_args: Vec<String>) -> CommandResult {
    println!("solve wordle!");
    Ok(())
}

fn main() -> CommandResult {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("play") => play(args.collect()),
        Some("solve") => solve(args.collect()),
        Some("help") | Some("-h") | Some(_) | None => help(),
    }.and(Ok(()))
}
