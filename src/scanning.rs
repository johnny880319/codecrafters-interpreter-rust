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
        match source.as_bytes()[offset] {
            b'(' => {
                tokens.push(Token {
                    token_type: "LEFT_PAREN".to_string(),
                    lexeme: "(".to_string(),
                    literal: None,
                });
                offset += 1;
            }
            b')' => {
                tokens.push(Token {
                    token_type: "RIGHT_PAREN".to_string(),
                    lexeme: ")".to_string(),
                    literal: None,
                });
                offset += 1;
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Unexpected character '{}' at offset {}",
                    source.as_bytes()[offset] as char,
                    offset
                ));
            }
        }
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
