use crate::scanning::{Token, TokenType};
use anyhow::Result;

pub struct AstNode {
    pub val: String,
    pub children: Vec<Self>,
}

pub fn build_ast(tokens: &[Token], mut offset: usize) -> Result<(Vec<AstNode>, usize)> {
    let mut ast_nodes = Vec::new();

    while offset < tokens.len() {
        let token = &tokens[offset];
        match token.kind {
            TokenType::LeftParen => {
                let (children, new_offset) = build_ast(tokens, offset + 1)?;
                ast_nodes.push(AstNode {
                    val: "group".to_string(),
                    children,
                });
                offset = new_offset;
            }
            TokenType::RightParen => {
                return Ok((ast_nodes, offset + 1));
            }
            _ => {
                if token.literal.is_some() {
                    ast_nodes.push(AstNode {
                        val: token.literal.as_deref().unwrap().to_string(),
                        children: Vec::new(),
                    });
                } else {
                    ast_nodes.push(AstNode {
                        val: token.lexeme.clone(),
                        children: Vec::new(),
                    });
                }
                offset += 1;
            }
        }
    }
    Ok((ast_nodes, offset))
}
