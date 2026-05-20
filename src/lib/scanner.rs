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

    // similar to self.advance(), but does not consume the character
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            Some('\0')
        } else {
            self.source.chars().nth(self.current)
        }
    }
    
    // if we find a slash we check for another slash
    // if present, we consume tokens until the end of the line
    // and return Skip
    //
    // otherwise we treat it as a division operator and return Slash
    fn handle_slash(&mut self) -> TokenType {
         if self.try_match('/') {
            while self.peek() != Some('\n')  && !self.is_at_end() {
                self.advance();
            }
            TokenType::Skip
        } else {
            TokenType::Slash
        }
    }
    
    fn handle_newline(&mut self) -> TokenType {
         self.line += 1;
         TokenType::Skip
    }

  // while (peek() != '"' && !isAtEnd()) {
  //     if (peek() == '\n') line++;
  //     advance();
  //   }
  //
  //   if (isAtEnd()) {
  //     Lox.error(line, "Unterminated string.");
  //     return;
  //   }
  //
  //   // The closing ".
  //   advance();
  //
  //   // Trim the surrounding quotes.
  //   String value = source.substring(start + 1, current - 1);
  //   addToken(STRING, value);

    fn handle_string(&mut self) -> TokenType {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') { 
                self.line += 1;
                self.advance();
            }
        }

        if self.is_at_end() {
            error::unterminated_string(self.line);
            return TokenType::Skip;
        }
    
        // advance past the closing '"'
        self.advance();

        let str_val = self.source[self.start + 1.. self.current-1].to_string();
        TokenType::String(str_val)

    }


    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {

            let token = match c {
                    ' ' => TokenType::Skip,
                    '\r' => TokenType::Skip,
                    '\t' => TokenType::Skip,
                    '\n' => self.handle_newline(),
                    '"' => self.handle_string(),
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '-' => TokenType::Minus,
                    '+' => TokenType::Plus,
                    ';' => TokenType::Semicolon,
                    '*' => TokenType::Star,
                    '!' => self.if_next_else('=', TokenType::BangEqual, TokenType::Bang ),
                    '=' => self.if_next_else('=', TokenType::EqualEqual, TokenType::Equal),
                    '<' => self.if_next_else('=', TokenType::LessEqual, TokenType::Less),
                    '>' => self.if_next_else('=', TokenType::GreaterEqual, TokenType::Greater),
                    '/' => self.handle_slash(),
                    _ => TokenType::Invalid,
            };
        
            match token {
                TokenType::Invalid => unexpected_character(self.line),
                TokenType::Skip => (),
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
