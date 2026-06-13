use anyhow::Result;

pub struct Token {
    pub token_type: String,
    pub lexeme: String,
    pub literal: Option<String>,
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>> {
    let mut offset = 0;
    let mut tokens = Vec::new();

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
            _ => {
                return Err(anyhow::anyhow!(
                    "Unexpected character '{}' at offset {}",
                    c,
                    offset
                ));
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
    Ok(tokens)
}

pub fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        println!(
            "{} {} {}",
            token.token_type,
            token.lexeme,
            token.literal.as_deref().unwrap_or("null")
        );
    }
}
