pub struct Token {
    pub token_type: String,
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
            '\n' => {
                line += 1;
                continue;
            }
            _ => {
                errors.push(ScanError { line, character: c });
                offset += 1;
                continue;
            }
        };

        tokens.push(Token {
            token_type: token_type.to_string(),
            lexeme: c.to_string(),
            literal: None,
        });
        offset += 1;
    }

    tokens.push(Token {
        token_type: "EOF".to_string(),
        lexeme: "".to_string(),
        literal: None,
    });
    (tokens, errors)
}

pub fn print_tokens(tokens: &[Token], errors: &[ScanError]) {
    for token in tokens {
        println!(
            "{} {} {}",
            token.token_type,
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
