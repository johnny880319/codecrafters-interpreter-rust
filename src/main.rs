use crate::scanning::{ScanError, Token};
use std::env;
use std::fs;

mod scanning;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let (tokens, errors) = scanning::scan_tokens(&file_contents).unwrap_or_else(|_| {
                eprintln!("Failed to scan tokens from file {filename}");
                (Vec::new(), Vec::new())
            });

            print_scan_results(&tokens, &errors);
            if !errors.is_empty() {
                std::process::exit(65);
            }
        }
        "parse" => {
            let (tokens, _) = scanning::scan_tokens(&file_contents).unwrap_or_else(|_| {
                eprintln!("Failed to scan tokens from file {filename}");
                (Vec::new(), Vec::new())
            });
            print_parse_results(&tokens);
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}

fn print_scan_results(tokens: &[Token], errors: &[ScanError]) {
    for token in tokens {
        println!(
            "{} {} {}",
            token.kind.as_str(),
            token.lexeme,
            token.literal.as_deref().unwrap_or("null")
        );
    }
    for error in errors {
        eprintln!("[line {}] Error: {}", error.line, error.message);
    }
}

fn print_parse_results(tokens: &[Token]) {
    for token in tokens {
        if token.literal.is_some() {
            println!("{}", token.literal.as_deref().unwrap());
        } else {
            println!("{}", token.lexeme);
        }
    }
}
