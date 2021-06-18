use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};

use errors::LoxError;
use lexer::{Token};

mod errors;
mod lexer;

fn main() {
    let mut args = args();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if args.len() == 1 {
        run_file(args.next().unwrap().as_str()).unwrap();
    } else {
        run_prompt().unwrap();
    }
}

fn run_file(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    if let Err(err) = run(&source){
        println!("{}", err);
    }
    
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
            if let Err(err) = run(&line){
                println!("{}", err);
            }
        }
    }
}

fn run(source: &str) -> Result<(), LoxError>{
    let tokens = Token::scan_tokens(source)?;

    Ok(())
}
