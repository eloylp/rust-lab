use std::io;

mod cli;
mod caesar;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match cli::exec::with(&args[1..], io::stdin().lock(), io::stdout()) {
        Err(error) => {
            println!("{}", error);
            std::process::exit(1)
        }
        _ => {}
    }
}