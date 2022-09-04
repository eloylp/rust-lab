use std::io;

use caesar::exec;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(error) = exec::with(&args[1..], io::stdin().lock(), io::stdout()) {
        println!("{}", error);
        std::process::exit(1)
    }
}
