use std::panic;

use crate::lib::literal::Literal;


#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    // literal: 
    // TODO literal: 
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literal(Literal),

  // Literals.
  Identifier(String), String(String), Number(f64),

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,
   
  // Special Processing States
  Invalid, Skip,

  Eof
}


impl TokenType {
    
    fn unwrap_string(&self) -> &str {
        match self {
            Self::String(s) => s,
            _ => panic!("attempted to unwrap a String from another token type"),
        }
    }

    fn unwrap_ident(&self) -> &str {
        match self {
            Self::Identifier(s) => s,
            _ => panic!("attempted to unwrap a Identifier from another token type"),
        }
    }

    fn unwrap_number(&self) -> &f64 {
        match self {
            Self::Number(n) => n,
            _ => panic!("attempted to unwrap a Number from another token type"),
        }
    }
}



