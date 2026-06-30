mod parsing;
mod scanning;

use crate::parsing::AstNode;
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
            let (ast_nodes, _) = parsing::build_ast(&tokens, 0).unwrap_or_else(|_| {
                eprintln!("Failed to build AST from tokens in file {filename}");
                (Vec::new(), 0)
            });
            print_parse_results(&ast_nodes);
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

fn print_parse_results(ast_nodes: &[AstNode]) {
    for node in ast_nodes {
        if !node.children.is_empty() {
            print!("(");
        }
        print!("{}", node.val);
        if !node.children.is_empty() {
            print!(" ");
        }
        print_parse_results(&node.children);
        if !node.children.is_empty() {
            print!(")");
        }
    }
}
