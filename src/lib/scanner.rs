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
        // set out the the current character
        let out = self.source.chars().nth(self.current);
        // increment current pointer
        self.current += 1;
        out
    }

    fn add_token(&mut self, token_type: TokenType) {
        let source_text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, source_text.to_string(), self.line));
    }

    fn try_match(&mut self, expected: char) -> bool {
        // false if at end of source
        if self.is_at_end() {
            return false;
        }
    
        // if the current char matches expected, increment current pointer to consume it
        // and return true
        match self.source.chars().nth(self.current) {
            Some(found) if found == expected => {
                self.current += 1;
                true
            }, 
            _ => false,
        }
    }
    

    fn if_next_else(&mut self, c: char, matched: TokenType, no_match: TokenType) -> TokenType {
        match self.try_match(c) {
            true => matched,
            false => no_match,
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
                    '!' => self.if_next_else('=', TokenType::BangEqual, TokenType::Bang ),
                    '=' => self.if_next_else('=', TokenType::EqualEqual, TokenType::Equal),
                    '<' => self.if_next_else('=', TokenType::LessEqual, TokenType::Less),
                    '>' => self.if_next_else('=', TokenType::GreaterEqual, TokenType::Greater),
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
