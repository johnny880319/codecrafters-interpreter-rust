use crate::scanning::Scanner;
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

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {filename}");
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens().unwrap_or_else(|_| {
                eprintln!("Failed to scan tokens from file {filename}");
            });
            scanner.print();

            if scanner.has_errors() {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
