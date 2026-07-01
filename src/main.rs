mod parsing;
mod scanning;

use crate::parsing::Expr;
use crate::scanning::{ScanError, Token};
use std::env;
use std::fs;

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
            let expr = parsing::parse_expression(&tokens).unwrap_or_else(|e| {
                eprintln!("Failed to build AST: {e}");
                std::process::exit(65);
            });
            print_parse_results(expr);
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

fn print_parse_results(expr: Expr) {
    match expr {
        Expr::Number(value) | Expr::String(value) => {
            print!("{value}");
        }
        Expr::Bool(value) => {
            print!("{value}");
        }
        Expr::Nil => {
            print!("nil");
        }
        Expr::Group(expr) => {
            print!("(group ");
            print_parse_results(*expr);
            print!(")");
        }
        Expr::Unary { operator, right } => {
            print!("({operator} ");
            print_parse_results(*right);
            print!(")");
        }
        Expr::Binary {
            operator,
            left,
            right,
        } => {
            print!("({operator} ");
            print_parse_results(*left);
            print!(" ");
            print_parse_results(*right);
            print!(")");
        }
    }
}
