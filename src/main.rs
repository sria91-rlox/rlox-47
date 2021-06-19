use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};

use errors::LoxError;
use lexer::Token;

mod errors;
mod lexer;

fn main() {
    let mut args = args();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let path = args.next().unwrap();
        println!("Running: {}", path);
        run_file(path.as_str()).unwrap();
    } else {
        println!("rlox prompt:");
        run_prompt().unwrap();
    }
}

fn run_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    run(&source);
    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    let stdin = stdin();
    let mut i = stdin.lock();
    let stdout = stdout();
    let mut o = stdout.lock();

    let mut line = String::new();

    loop {
        o.write_all(b"> ")?;

        i.read_line(&mut line)?;

        if !line.is_empty() {
            run(&line);
        }
    }
}

fn run(source: &str) {
    if let Err(errors) = Token::scan_tokens(source) {
        for err in &errors {
            println!("{}", err);
        }
    }
}
