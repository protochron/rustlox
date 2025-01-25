use std::fs::read;
use std::io::BufRead;

mod errors;
mod scanner;
mod tokens;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let l = line?;
        run(l.into_bytes());
    }
    Ok(())
}

fn run_file(path: &str) -> anyhow::Result<()> {
    let b = read(path)?;
    run(b);
    Ok(())
}

fn run(bytes: Vec<u8>) {}
