mod error;
mod interpreter;
mod parser;
mod tokenizer;

use error::{clear_errors, code_error, ExitCode};
use interpreter::INTERPRETER;
use parser::debug::output_tree;
use parser::statement::Statement;
use parser::Parser;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::process;
use tokenizer::debug::output_tokens;
use tokenizer::token::Token;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: seth [file]");
        process::exit(ExitCode::USAGE as i32)
    } else if args.len() == 2 {
        run_file(&args[0])
    } else {
        run_prompt();
    }
}

fn run_file(filepath: &String) {
    let file: File = match File::open(&filepath) {
        Ok(f) => f,
        _ => {
            println!("File not found: {}", &filepath);
            process::exit(ExitCode::NOINPUT as i32)
        }
    };

    let mut reader: BufReader<File> = BufReader::new(file);
    let mut code: String = String::new();

    match reader.read_to_string(&mut code) {
        Ok(size) => {
            if size == 0 {
                process::exit(ExitCode::OK as i32)
            }
        }
        _ => {
            println!("Unable to read file: {}", &filepath);
            process::exit(ExitCode::NOINPUT as i32)
        }
    }

    run(code);

    match code_error() {
        ExitCode::OK => (),
        c => process::exit(c as i32),
    }
}

fn run_prompt() {
    loop {
        print!("> ");

        match stdout().flush() {
            Ok(_) => (),
            _ => {
                println!("Couldn't write to standard output");
                process::exit(ExitCode::IOERR as i32)
            }
        }

        let mut code: String = String::new();

        match stdin().read_line(&mut code) {
            Ok(size) => {
                if size == 0 {
                    process::exit(ExitCode::OK as i32)
                }
            }
            _ => {
                println!("Couldn't read to standard input");
                process::exit(ExitCode::IOERR as i32)
            }
        }

        run(code);
        clear_errors();
    }
}

fn run(code: String) {
    let tokens: Vec<Token> = Tokenizer::new(code).tokenize();
    println!("{}", output_tokens(&tokens));

    let statements: Vec<Statement> = Parser::new(tokens).parse();
    println!("{}", output_tree(&statements));

    INTERPRETER.interpret(statements);
}
