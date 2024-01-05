mod error;
mod expression;
mod interpreter;
mod parser;
mod scanner;
mod token;

use expression::{Expression, Statement};
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: seth [file]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[0].clone());
    } else {
        run_prompt();
    }
}

fn run_file(filename: String) {
    let file: File = File::open(&filename).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);

    loop {
        let mut line: String = String::new();
        let size: usize = reader.read_line(&mut line).unwrap();

        // EOF
        if size == 0 {
            break;
        }

        run(line);

        // Found an error in the code.
        unsafe {
            if error::HAD_ERROR {
                process::exit(65);
            }
            if error::HAD_RUNTIME_ERROR {
                process::exit(70);
            }
        }
    }
}

fn run_prompt() {
    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut line: String = String::new();

        io::stdin().read_line(&mut line).unwrap();

        if line.eq("") {
            break;
        }

        run(line);

        // Interactive mode shouldn't stop when error occurs.
        unsafe {
            error::HAD_ERROR = false;
            error::HAD_RUNTIME_ERROR = false;
        }
    }
}

fn run(code: String) {
    let scanner: Scanner = Scanner::new(code);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Statement> = parser.parse();

    unsafe {
        if error::HAD_ERROR == true {
            return;
        }
    }

    let interpreter: Interpreter = Interpreter::new();
    interpreter.interpret(statements);
}
