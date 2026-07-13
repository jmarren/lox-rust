use crate::lib::literal::Literal;


#[derive(Debug)]
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


#[derive(Debug, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  Literal(Literal),

  // Literals.
  // Identifier(String), String(String), Number(f64),

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,
   
  // Special Processing States
  Invalid, Skip,



  Eof
}



