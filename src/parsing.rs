use crate::scanning::{Token, TokenType};
use anyhow::Result;

pub struct AstNode {
    pub val: String,
    pub children: Vec<Self>,
}

pub fn parse_expression(tokens: &[Token]) -> Result<AstNode> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }
    if tokens[tokens.len() - 1].kind == TokenType::Eof {
        return parse_term(&tokens[..tokens.len() - 1]);
    }
    parse_term(tokens)
}

fn parse_term(tokens: &[Token]) -> Result<AstNode> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    let mut parentheses_depth = 0;
    let mut mid = tokens.len();

    while mid > 0 {
        mid -= 1;
        match tokens[mid].kind {
            TokenType::RightParen => {
                parentheses_depth += 1;
            }
            TokenType::LeftParen => {
                parentheses_depth -= 1;
            }
            TokenType::Plus | TokenType::Minus if parentheses_depth == 0 => {
                if mid == 0 || !is_expression(&tokens[mid - 1]) {
                    continue;
                }
                let left_node = parse_term(&tokens[0..mid])?;
                let right_node = parse_factor(&tokens[mid + 1..])?;
                return Ok(AstNode {
                    val: tokens[mid].lexeme.clone(),
                    children: vec![left_node, right_node],
                });
            }
            _ => {}
        }
    }
    parse_factor(tokens)
}

fn parse_factor(tokens: &[Token]) -> Result<AstNode> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    let mut parentheses_depth = 0;
    let mut mid = tokens.len();

    while mid > 0 {
        mid -= 1;
        match tokens[mid].kind {
            TokenType::RightParen => {
                parentheses_depth += 1;
            }
            TokenType::LeftParen => {
                parentheses_depth -= 1;
            }
            TokenType::Star | TokenType::Slash if parentheses_depth == 0 => {
                let left_node = parse_factor(&tokens[0..mid])?;
                let right_node = parse_unary(&tokens[mid + 1..])?;
                return Ok(AstNode {
                    val: tokens[mid].lexeme.clone(),
                    children: vec![left_node, right_node],
                });
            }
            _ => {}
        }
    }
    parse_unary(tokens)
}

fn parse_unary(tokens: &[Token]) -> Result<AstNode> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    match tokens[0].kind {
        TokenType::Bang | TokenType::Minus => {
            let child_node = parse_unary(&tokens[1..])?;
            Ok(AstNode {
                val: tokens[0].lexeme.clone(),
                children: vec![child_node],
            })
        }
        _ => parse_primary(tokens),
    }
}

fn parse_primary(tokens: &[Token]) -> Result<AstNode> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    let n = tokens.len();

    match tokens[0].kind {
        TokenType::Number | TokenType::String if n == 1 => Ok(AstNode {
            val: tokens[0].literal.as_deref().unwrap_or("").to_string(),
            children: Vec::new(),
        }),
        TokenType::True | TokenType::False | TokenType::Nil | TokenType::Identifier if n == 1 => {
            Ok(AstNode {
                val: tokens[0].lexeme.clone(),
                children: Vec::new(),
            })
        }
        TokenType::LeftParen if tokens[n - 1].kind == TokenType::RightParen => Ok(AstNode {
            val: "group".to_string(),
            children: vec![parse_expression(&tokens[1..tokens.len() - 1])?],
        }),
        _ => Err(anyhow::anyhow!("Unexpected primary tokens: {:?}", tokens)),
    }
}

const fn is_expression(token: &Token) -> bool {
    matches!(
        token.kind,
        TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::RightParen
            | TokenType::Identifier
    )
}
