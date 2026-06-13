use anyhow::Result;

pub struct Token {
    pub kind: String,
    pub lexeme: String,
    pub literal: Option<String>,
}

pub struct ScanError {
    line: usize,
    character: char,
}

pub fn scan_tokens(source: &str) -> Result<(Vec<Token>, Vec<ScanError>)> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut line = 1;

    let mut offset = 0;
    while offset < source.len() {
        let c = source.as_bytes()[offset] as char;

        let (token_type, lexeme, new_offset) = match c {
            ',' | '.' | '-' | '+' | ';' | '*' | '(' | ')' | '{' | '}' => {
                scan_single_character_token(source, offset)?
            }
            '=' | '!' | '<' | '>' => scan_equal_operator(source, offset)?,
            '/' => scan_slash(source, offset)?,
            ' ' | '\t' => (String::new(), String::new(), offset + 1),
            '\n' => {
                line += 1;
                (String::new(), String::new(), offset + 1)
            }
            _ => {
                errors.push(ScanError { line, character: c });
                (String::new(), String::new(), offset + 1)
            }
        };
        offset = new_offset;

        if lexeme.is_empty() {
            continue;
        }
        tokens.push(Token {
            kind: token_type,
            lexeme,
            literal: None,
        });
    }

    tokens.push(Token {
        kind: "EOF".to_string(),
        lexeme: String::new(),
        literal: None,
    });
    Ok((tokens, errors))
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

fn scan_single_character_token(source: &str, offset: usize) -> Result<(String, String, usize)> {
    let c = source.as_bytes()[offset] as char;
    let token_type = match c {
        ',' => "COMMA",
        '.' => "DOT",
        '-' => "MINUS",
        '+' => "PLUS",
        ';' => "SEMICOLON",
        '*' => "STAR",
        '(' => "LEFT_PAREN",
        ')' => "RIGHT_PAREN",
        '{' => "LEFT_BRACE",
        '}' => "RIGHT_BRACE",
        _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
    }
    .to_owned();
    Ok((token_type, c.to_string(), offset + 1))
}

fn scan_equal_operator(source: &str, mut offset: usize) -> Result<(String, String, usize)> {
    let c = source.as_bytes()[offset] as char;
    let mut lexeme = c.to_string();

    let mut token_type = match c {
        '=' => "EQUAL",
        '!' => "BANG",
        '<' => "LESS",
        '>' => "GREATER",
        _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
    }
    .to_owned();
    offset += 1;

    if offset < source.len() && source.as_bytes()[offset] as char == '=' {
        offset += 1;
        lexeme += "=";
        token_type += "_EQUAL";
    }
    Ok((token_type, lexeme, offset))
}

fn scan_slash(source: &str, offset: usize) -> Result<(String, String, usize)> {
    if offset >= source.len() || source.as_bytes()[offset] as char != '/' {
        return Err(anyhow::anyhow!("Expected '/' at offset {}", offset,));
    }
    if offset + 1 < source.len() && source.as_bytes()[offset + 1] as char == '/' {
        return Ok((String::new(), String::new(), source.len()));
    }
    Ok(("SLASH".to_string(), "/".to_string(), offset + 1))
}
