#![allow(dead_code)]

use anyhow::Result;
use crate::app_error::AppError;

pub const TOKEN_LENGTH: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenType {
    Answer,
    Game,
    GameTemplate,
    Invalid,
}

impl TokenType {
    pub fn leading_byte(&self) -> u8 {
        match self {
            TokenType::Answer => 'a' as u8,
            TokenType::Game => 'g' as u8,
            TokenType::GameTemplate => 't' as u8,
            TokenType::Invalid => 'i' as u8,
        }
    }
    pub fn get_token_type(token: &Token) -> TokenType {
        match token.token[0] as char {
            'a' => TokenType::Answer,
            'g' => TokenType::Game,
            't' => TokenType::GameTemplate,
            _ => TokenType::Invalid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Token {
    pub token: [u8; TOKEN_LENGTH],
}

impl Token {
    pub fn from_string(token_str: &str) -> Result<Self, AppError> {
        let bytes = token_str.bytes();
        if bytes.len() == TOKEN_LENGTH {
            let mut token = [0u8; TOKEN_LENGTH];
            for (i, b) in bytes.enumerate() {
                token[i] = b;
            }
            if TokenType::get_token_type(&Self { token }) == TokenType::Invalid {
                return Err(AppError::TokenError("invalid token".to_string()));
            }
            return Ok(Self { token });
        }
        Err(AppError::TokenError("no token".to_string()))
    }

    pub fn get_token_type(&self) -> TokenType {
        TokenType::get_token_type(self)
    }
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.token).to_string()
    }
    pub fn to_str(&self) -> &str {
        std::str::from_utf8(&self.token).unwrap()
    }
}