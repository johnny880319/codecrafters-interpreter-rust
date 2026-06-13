pub struct Token {
    pub kind: String,
    pub lexeme: String,
    pub literal: Option<String>,
}

pub struct ScanError {
    line: usize,
    character: char,
}

pub fn scan_tokens(source: &str) -> (Vec<Token>, Vec<ScanError>) {
    let mut offset = 0;
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut line = 1;

    while offset < source.len() {
        let c = source.as_bytes()[offset] as char;
        let mut lexeme = c.to_string();

        let token_type = match c {
            ',' => "COMMA",
            '.' => "DOT",
            '-' => "MINUS",
            '+' => "PLUS",
            ';' => "SEMICOLON",
            '/' => "SLASH",
            '*' => "STAR",
            '(' => "LEFT_PAREN",
            ')' => "RIGHT_PAREN",
            '{' => "LEFT_BRACE",
            '}' => "RIGHT_BRACE",
            '=' => {
                if offset + 1 < source.len() && source.as_bytes()[offset + 1] as char == '=' {
                    offset += 1;
                    lexeme = "==".to_string();
                    "EQUAL_EQUAL"
                } else {
                    "EQUAL"
                }
            }
            '!' => {
                if offset + 1 < source.len() && source.as_bytes()[offset + 1] as char == '=' {
                    offset += 1;
                    lexeme = "!=".to_string();
                    "BANG_EQUAL"
                } else {
                    "BANG"
                }
            }
            '\n' => {
                line += 1;
                offset += 1;
                continue;
            }
            _ => {
                errors.push(ScanError { line, character: c });
                offset += 1;
                continue;
            }
        };
        offset += 1;

        tokens.push(Token {
            kind: token_type.to_string(),
            lexeme,
            literal: None,
        });
    }

    tokens.push(Token {
        kind: "EOF".to_string(),
        lexeme: String::new(),
        literal: None,
    });
    (tokens, errors)
}

pub fn print_tokens(tokens: &[Token], errors: &[ScanError]) {
    for token in tokens {
        println!(
            "{} {} {}",
            token.kind,
            token.lexeme,
            token.literal.as_deref().unwrap_or("null")
        );
    }
    for error in errors {
        eprintln!(
            "[line {}] Error: Unexpected character: {}",
            error.line, error.character
        );
    }
}
