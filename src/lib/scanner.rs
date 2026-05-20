use crate::lib::{error::{self, unexpected_character}, token::{Token, TokenType}};



pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize, 
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens() -> Vec<Token> {
        loop {
            
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let source_text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, source_text.to_string(), self.line));
    }

    fn try_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if let Some(found) = self.source.chars().nth(self.current) && found != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }




    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {

                
                let token = match c {
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '-' => TokenType::Minus,
                    '+' => TokenType::Plus,
                    ';' => TokenType::Semicolon,
                    '/' => TokenType::Slash,
                    '*' => TokenType::Star,
                    '!' => {
                        match self.try_match('=') {
                            true => TokenType::BangEqual,
                            false => TokenType::Bang,
                        }
                    },
                    '=' => {
                        match self.try_match('=') {
                            true => TokenType::EqualEqual,
                            false => TokenType::Equal,
                        }
                    },
                    '<' => {
                        match self.try_match('=') {
                            true => TokenType::LessEqual,
                            false => TokenType::Less,
                        }
                    },
                    '>' => {
                        match self.try_match('=') {
                            true => TokenType::GreaterEqual,
                            false => TokenType::Greater,
                        }
                    }
                    _ => TokenType::Invalid,
            };
        
            match token {
                TokenType::Invalid => unexpected_character(self.line),
                _ => self.add_token(token),
            };

        }
    }
}

  // List<Token> scanTokens() {
  //   while (!isAtEnd()) {
  //     // We are at the beginning of the next lexeme.
  //     start = current;
  //     scanToken();
  //   }
  //
  //   tokens.add(new Token(EOF, "", null, line));
  //   return tokens;
  // }
  //
// class Scanner {
//   private final String source;
//   private final List<Token> tokens = new ArrayList<>();
//
//   Scanner(String source) {
//     this.source = source;
//   }
// }
