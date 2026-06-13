use anyhow::Result;

pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
    offset: usize,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
}

struct Token {
    pub kind: String,
    pub lexeme: String,
    pub literal: Option<String>,
}

struct ScanError {
    line: usize,
    message: String,
}

impl Scanner<'_> {
    pub const fn new(source: &str) -> Scanner<'_> {
        Scanner {
            source,
            line: 1,
            offset: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn print(&self) {
        for token in &self.tokens {
            println!(
                "{} {} {}",
                token.kind,
                token.lexeme,
                token.literal.as_deref().unwrap_or("null")
            );
        }
        for error in &self.errors {
            eprintln!("[line {}] Error: {}", error.line, error.message);
        }
    }

    pub const fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn scan_tokens(&mut self) -> Result<()> {
        while self.offset < self.source.len() {
            self.scan_once()?;
        }
        self.tokens.push(Token {
            kind: "EOF".to_string(),
            lexeme: String::new(),
            literal: None,
        });
        Ok(())
    }

    fn scan_once(&mut self) -> Result<()> {
        let c = self.source.as_bytes()[self.offset] as char;

        match c {
            ',' | '.' | '-' | '+' | ';' | '*' | '(' | ')' | '{' | '}' => {
                self.scan_single_character_token()
            }
            '=' | '!' | '<' | '>' => self.scan_equal_operator(),
            '/' => self.scan_slash(),
            ' ' | '\t' => {
                self.offset += 1;
                Ok(())
            }
            '\n' => {
                self.line += 1;
                self.offset += 1;
                Ok(())
            }
            _ => {
                self.errors.push(ScanError {
                    line: self.line,
                    message: format!("Unexpected character: {c}"),
                });
                self.offset += 1;
                Ok(())
            }
        }
    }

    fn scan_single_character_token(&mut self) -> Result<()> {
        let c = self.source.as_bytes()[self.offset] as char;
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

        self.tokens.push(Token {
            kind: token_type,
            lexeme: c.to_string(),
            literal: None,
        });
        self.offset += 1;

        Ok(())
    }

    fn scan_equal_operator(&mut self) -> Result<()> {
        let c = self.source.as_bytes()[self.offset] as char;
        let mut lexeme = c.to_string();

        let mut token_type = match c {
            '=' => "EQUAL",
            '!' => "BANG",
            '<' => "LESS",
            '>' => "GREATER",
            _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
        }
        .to_owned();
        self.offset += 1;

        if self.offset < self.source.len() && self.source.as_bytes()[self.offset] as char == '=' {
            self.offset += 1;
            lexeme += "=";
            token_type += "_EQUAL";
        }

        self.tokens.push(Token {
            kind: token_type,
            lexeme,
            literal: None,
        });
        Ok(())
    }

    fn scan_slash(&mut self) -> Result<()> {
        if self.offset >= self.source.len() || self.source.as_bytes()[self.offset] as char != '/' {
            return Err(anyhow::anyhow!("Expected '/' at offset {}", self.offset,));
        }
        if self.offset + 1 < self.source.len()
            && self.source.as_bytes()[self.offset + 1] as char == '/'
        {
            // find newline or end of file
            let mut new_offset = self.offset + 2;
            while new_offset < self.source.len()
                && self.source.as_bytes()[new_offset] as char != '\n'
            {
                new_offset += 1;
            }
            self.offset = new_offset;
            return Ok(());
        }

        self.offset += 1;
        self.tokens.push(Token {
            kind: "SLASH".to_string(),
            lexeme: "/".to_string(),
            literal: None,
        });
        Ok(())
    }
}

// fn scan_string(
//     source: &str,
//     mut offset: usize,
//     errors: &mut Vec<ScanError>,
// ) -> Result<(Token, usize)> {
//     if offset >= source.len() || source.as_bytes()[offset] as char != '"' {
//         return Err(anyhow::anyhow!("Expected '\"' at offset {}", offset,));
//     }

//     offset += 1;
//     let start_offset = offset;
//     while offset < source.len() && source.as_bytes()[offset] as char != '"' {
//         offset += 1;
//     }

//     if offset >= source.len() {
//         errors.push(ScanError {
//             line:

//     return Ok((
//         Token {
//             kind: "STRING".to_string(),
//             lexeme: source[start_offset - 1..offset + 1].to_string(),
//             literal: Some(source[start_offset..offset].to_string()),
//         },
//         offset + 1,
//     ));
// }}
