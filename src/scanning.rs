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
        self.offset += 1;

        match c {
            ',' | '.' | '-' | '+' | ';' | '*' | '(' | ')' | '{' | '}' => {
                self.scan_single_character_token(c)?;
            }
            '_' | 'a'..='z' | 'A'..='Z' => self.scan_identifier(c),
            '=' | '!' | '<' | '>' => self.scan_equal_operator(c)?,
            ' ' | '\t' | '\n' => self.scan_space(c),
            '0'..='9' => self.scan_number(c),
            '"' => self.scan_string(c),
            '/' => self.scan_slash(c),
            _ => self.unexpected_character(c),
        }
        Ok(())
    }

    fn scan_single_character_token(&mut self, c: char) -> Result<()> {
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

        Ok(())
    }

    fn scan_identifier(&mut self, c: char) {
        let mut lexeme = c.to_string();

        while self.offset < self.source.len() {
            let next_char = self.source.as_bytes()[self.offset] as char;
            if next_char == '_' || next_char.is_ascii_alphanumeric() {
                lexeme += &next_char.to_string();
                self.offset += 1;
            } else {
                break;
            }
        }

        self.tokens.push(Token {
            kind: "IDENTIFIER".to_string(),
            lexeme,
            literal: None,
        });
    }

    fn scan_equal_operator(&mut self, c: char) -> Result<()> {
        let mut lexeme = c.to_string();

        let mut token_type = match c {
            '=' => "EQUAL",
            '!' => "BANG",
            '<' => "LESS",
            '>' => "GREATER",
            _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
        }
        .to_owned();

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

    const fn scan_space(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
        }
    }

    fn scan_number(&mut self, c: char) {
        let mut num_string = c.to_string();

        let mut has_dot = false;
        while self.offset < self.source.len() {
            let next_char = self.source.as_bytes()[self.offset] as char;
            if next_char == '.' && !has_dot {
                has_dot = true;
                num_string += ".";
                self.offset += 1;
            } else if next_char.is_ascii_digit() {
                num_string += &next_char.to_string();
                self.offset += 1;
            } else {
                break;
            }
        }

        let mut literal = num_string.clone();
        if !has_dot {
            literal += ".0";
        }

        while literal.ends_with("00") {
            literal.pop();
        }

        self.tokens.push(Token {
            kind: "NUMBER".to_string(),
            lexeme: num_string.clone(),
            literal: Some(literal),
        });
    }

    fn scan_string(&mut self, _: char) {
        let start_offset = self.offset;
        while self.offset < self.source.len() && self.source.as_bytes()[self.offset] as char != '"'
        {
            self.offset += 1;
        }

        if self.offset >= self.source.len() {
            self.errors.push(ScanError {
                line: self.line,
                message: "Unterminated string.".to_string(),
            });
            return;
        }

        self.tokens.push(Token {
            kind: "STRING".to_string(),
            lexeme: self.source[(start_offset - 1)..=self.offset].to_string(),
            literal: Some(self.source[start_offset..self.offset].to_string()),
        });

        self.offset += 1;
    }

    fn scan_slash(&mut self, _: char) {
        if self.offset < self.source.len() && self.source.as_bytes()[self.offset] as char == '/' {
            // find newline or end of file
            let mut new_offset = self.offset + 1;
            while new_offset < self.source.len()
                && self.source.as_bytes()[new_offset] as char != '\n'
            {
                new_offset += 1;
            }
            self.offset = new_offset;
            return;
        }

        self.offset += 1;

        self.tokens.push(Token {
            kind: "SLASH".to_string(),
            lexeme: "/".to_string(),
            literal: None,
        });
    }

    fn unexpected_character(&mut self, c: char) {
        self.errors.push(ScanError {
            line: self.line,
            message: format!("Unexpected character: {c}"),
        });
    }
}
