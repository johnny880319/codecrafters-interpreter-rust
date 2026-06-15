use anyhow::Result;

pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
    offset: usize,
    tokens: Vec<Token>,
    errors: Vec<ScanError>,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
}

#[derive(Clone)]
pub struct ScanError {
    pub line: usize,
    pub message: String,
}

pub fn scan_tokens(source: &str) -> Result<(Vec<Token>, Vec<ScanError>)> {
    let mut scanner = Scanner {
        source,
        line: 1,
        offset: 0,
        tokens: Vec::new(),
        errors: Vec::new(),
    };

    while scanner.offset < scanner.source.len() {
        scanner.scan_once()?;
    }
    scanner.tokens.push(Token {
        kind: TokenType::Eof,
        lexeme: String::new(),
        literal: None,
    });
    Ok((scanner.tokens, scanner.errors))
}

impl Scanner<'_> {
    fn scan_once(&mut self) -> Result<()> {
        let c = self.source.as_bytes()[self.offset] as char;
        self.offset += 1;

        match c {
            ',' | '.' | '-' | '+' | ';' | '*' | '(' | ')' | '{' | '}' => {
                self.scan_single_character(c)?;
            }
            '=' | '!' | '<' | '>' => self.scan_one_or_two_character(c)?,
            '_' | 'a'..='z' | 'A'..='Z' => self.scan_keywords_and_identifier(c),
            '0'..='9' => self.scan_number(c),
            '"' => self.scan_string(c),
            '/' => self.scan_slash(c),
            ' ' | '\t' | '\n' => self.scan_space(c),
            _ => self.unexpected_character(c),
        }
        Ok(())
    }

    fn scan_single_character(&mut self, c: char) -> Result<()> {
        let token_type = match c {
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
        };

        self.tokens.push(Token {
            kind: token_type,
            lexeme: c.to_string(),
            literal: None,
        });

        Ok(())
    }

    fn scan_one_or_two_character(&mut self, c: char) -> Result<()> {
        let mut lexeme = c.to_string();

        let mut token_len = 1;
        if self.offset < self.source.len() && self.source.as_bytes()[self.offset] as char == '=' {
            self.offset += 1;
            token_len += 1;
            lexeme += "=";
        }

        let token_type = match (c, token_len) {
            ('=', 1) => TokenType::Equal,
            ('!', 1) => TokenType::Bang,
            ('<', 1) => TokenType::Less,
            ('>', 1) => TokenType::Greater,
            ('=', 2) => TokenType::EqualEqual,
            ('!', 2) => TokenType::BangEqual,
            ('<', 2) => TokenType::LessEqual,
            ('>', 2) => TokenType::GreaterEqual,
            _ => return Err(anyhow::anyhow!("Unexpected character: {}", c)),
        };

        self.tokens.push(Token {
            kind: token_type,
            lexeme,
            literal: None,
        });
        Ok(())
    }

    fn scan_keywords_and_identifier(&mut self, c: char) {
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

        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.tokens.push(Token {
            kind: token_type,
            lexeme,
            literal: None,
        });
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

        if literal.ends_with('.') {
            literal += "0";
            num_string.pop();
            self.offset -= 1;
        }

        self.tokens.push(Token {
            kind: TokenType::Number,
            lexeme: num_string.clone(),
            literal: Some(literal),
        });
    }

    fn scan_string(&mut self, _: char) {
        let start_offset = self.offset;
        while self.offset < self.source.len() && self.source.as_bytes()[self.offset] as char != '"'
        {
            if self.source.as_bytes()[self.offset] as char == '\n' {
                self.line += 1;
            }
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
            kind: TokenType::String,
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

        self.tokens.push(Token {
            kind: TokenType::Slash,
            lexeme: "/".to_string(),
            literal: None,
        });
    }

    const fn scan_space(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
        }
    }

    fn unexpected_character(&mut self, c: char) {
        self.errors.push(ScanError {
            line: self.line,
            message: format!("Unexpected character: {c}"),
        });
    }
}

#[derive(Clone)]
pub enum TokenType {
    // Single-character tokens
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // One or two character tokens
    Equal,
    Bang,
    Less,
    Greater,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Literals
    Identifier,
    String,
    Number,

    // Slash
    Slash,

    // EOF
    Eof,
}

impl TokenType {
    pub const fn as_str(&self) -> &str {
        match self {
            // Single-character tokens.
            Self::Comma => "COMMA",
            Self::Dot => "DOT",
            Self::Minus => "MINUS",
            Self::Plus => "PLUS",
            Self::Semicolon => "SEMICOLON",
            Self::Star => "STAR",
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",

            // One or two character tokens
            Self::Equal => "EQUAL",
            Self::Bang => "BANG",
            Self::Less => "LESS",
            Self::Greater => "GREATER",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::BangEqual => "BANG_EQUAL",
            Self::LessEqual => "LESS_EQUAL",
            Self::GreaterEqual => "GREATER_EQUAL",

            // Keywords
            Self::And => "AND",
            Self::Class => "CLASS",
            Self::Else => "ELSE",
            Self::False => "FALSE",
            Self::For => "FOR",
            Self::Fun => "FUN",
            Self::If => "IF",
            Self::Nil => "NIL",
            Self::Or => "OR",
            Self::Print => "PRINT",
            Self::Return => "RETURN",
            Self::Super => "SUPER",
            Self::This => "THIS",
            Self::True => "TRUE",
            Self::Var => "VAR",
            Self::While => "WHILE",

            // Literals
            Self::Identifier => "IDENTIFIER",
            Self::String => "STRING",
            Self::Number => "NUMBER",

            // Slash
            Self::Slash => "SLASH",

            // EOF
            Self::Eof => "EOF",
        }
    }
}
