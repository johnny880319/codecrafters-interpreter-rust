use crate::scanning::{Token, TokenType};
use anyhow::Result;

pub enum Expr {
    Number(String),
    String(String),
    Bool(bool),
    Nil,
    Group(Box<Self>),
    Unary {
        operator: String,
        right: Box<Self>,
    },
    Binary {
        operator: String,
        left: Box<Self>,
        right: Box<Self>,
    },
}

pub fn parse_expression(tokens: &[Token]) -> Result<Expr> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }
    if tokens[tokens.len() - 1].kind == TokenType::Eof {
        return parse_equality(&tokens[..tokens.len() - 1]);
    }
    parse_equality(tokens)
}

fn parse_equality(tokens: &[Token]) -> Result<Expr> {
    if let Some(mid) =
        find_rightmost_operator(tokens, &[TokenType::EqualEqual, TokenType::BangEqual])
    {
        let left = parse_equality(&tokens[0..mid])?;
        let right = parse_comparison(&tokens[mid + 1..])?;
        return Ok(Expr::Binary {
            operator: tokens[mid].lexeme.clone(),
            left: Box::new(left),
            right: Box::new(right),
        });
    }
    parse_comparison(tokens)
}

fn parse_comparison(tokens: &[Token]) -> Result<Expr> {
    if let Some(mid) = find_rightmost_operator(
        tokens,
        &[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ],
    ) {
        let left = parse_comparison(&tokens[0..mid])?;
        let right = parse_term(&tokens[mid + 1..])?;
        return Ok(Expr::Binary {
            operator: tokens[mid].lexeme.clone(),
            left: Box::new(left),
            right: Box::new(right),
        });
    }
    parse_term(tokens)
}

fn parse_term(tokens: &[Token]) -> Result<Expr> {
    if let Some(mid) = find_rightmost_operator(tokens, &[TokenType::Plus, TokenType::Minus]) {
        let left = parse_term(&tokens[0..mid])?;
        let right = parse_factor(&tokens[mid + 1..])?;
        return Ok(Expr::Binary {
            operator: tokens[mid].lexeme.clone(),
            left: Box::new(left),
            right: Box::new(right),
        });
    }
    parse_factor(tokens)
}

fn parse_factor(tokens: &[Token]) -> Result<Expr> {
    if let Some(mid) = find_rightmost_operator(tokens, &[TokenType::Star, TokenType::Slash]) {
        let left = parse_factor(&tokens[0..mid])?;
        let right = parse_unary(&tokens[mid + 1..])?;
        return Ok(Expr::Binary {
            operator: tokens[mid].lexeme.clone(),
            left: Box::new(left),
            right: Box::new(right),
        });
    }
    parse_unary(tokens)
}

fn parse_unary(tokens: &[Token]) -> Result<Expr> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    match tokens[0].kind {
        TokenType::Bang | TokenType::Minus => {
            let child_node = parse_unary(&tokens[1..])?;
            Ok(Expr::Unary {
                operator: tokens[0].lexeme.clone(),
                right: Box::new(child_node),
            })
        }
        _ => parse_primary(tokens),
    }
}

fn parse_primary(tokens: &[Token]) -> Result<Expr> {
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No tokens to parse"));
    }

    let n = tokens.len();

    match tokens[0].kind {
        TokenType::LeftParen if tokens[n - 1].kind == TokenType::RightParen => {
            Ok(Expr::Group(Box::new(parse_expression(&tokens[1..n - 1])?)))
        }
        _ if n != 1 => Err(anyhow::anyhow!("Unexpected primary tokens: {:?}", tokens)),
        TokenType::Number => {
            let value = tokens[0]
                .literal
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Expected literal for number token"))?;
            Ok(Expr::Number(value.clone()))
        }
        TokenType::String => {
            let value = tokens[0]
                .literal
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Expected literal for string token"))?;
            Ok(Expr::String(value.clone()))
        }
        TokenType::True => Ok(Expr::Bool(true)),
        TokenType::False => Ok(Expr::Bool(false)),
        TokenType::Nil => Ok(Expr::Nil),
        _ => Err(anyhow::anyhow!("Unexpected primary tokens: {:?}", tokens)),
    }
}

fn find_rightmost_operator(tokens: &[Token], operators: &[TokenType]) -> Option<usize> {
    if tokens.is_empty() {
        return None;
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
            ref k if parentheses_depth == 0 && operators.contains(k) => {
                if mid == 0 || !is_expression(&tokens[mid - 1]) {
                    continue;
                }
                return Some(mid);
            }
            _ => {}
        }
    }
    None
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
