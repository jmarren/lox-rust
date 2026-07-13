use crate::lib::{error::{self, unexpected_character}, token::{Token, TokenType}};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize, 
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));
        &self.tokens
    }
    
    /// whether current is at end of source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// returns the current character and moves current forward by 1
    fn advance(&mut self) -> Option<char> {
        // set out the the current character
        let out = self.source.chars().nth(self.current);
        // increment current pointer
        self.current += 1;
        out
    }

    /// get the source contents from self.start to self.current, retrieve its token type,
    /// and push it in to self.tokens
    fn add_token(&mut self, token_type: TokenType) {
        let source_text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, source_text.to_string(), self.line));
    }
    
    /// If the current char matches expected, increments current pointer to consume it,
    /// then returns true. Otherwise returns false.
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
    
    /// Tries to match the provided character.
    /// If it is matched the matched type is returned and the current pointer is incremented to 
    /// consume it.
    /// otherwise the no_match type is returned
    fn if_next_else(&mut self, c: char, matched: TokenType, no_match: TokenType) -> TokenType {
        match self.try_match(c) {
            true => matched,
            false => no_match,
        }
    }

    /// returns the current character or '\0' if None
    /// 
    /// does NOT increment current pointer
    fn peek(&self) -> char {
        self.source
            .chars()
            .nth(self.current)
            .unwrap_or('\0')
    }
    
    /// returns the next character or '\0' if None
    /// 
    /// does NOT alter current pointer
    fn peek_next(&self) -> char {
        self.source
            .chars()
            .nth(self.current + 1)
            .unwrap_or('\0')
    }

    /// if we find a slash we check for another slash (meaning comment)
    /// if present, we consume tokens until the end of the line
    /// and return Skip
    ///
    /// otherwise we treat it as a division operator and return TokenType::Slash
    fn handle_slash(&mut self) -> TokenType {
         // if we match another slash,
         // advance until newline
         if self.try_match('/') {
             // (?) Should this use handle_newline? 
            while self.peek() != '\n'  && !self.is_at_end() {
                self.advance();
            }
            TokenType::Skip
        } else {
            TokenType::Slash
        }
    }
    
    /// Increments self.line and returns TokenType::Skip
    fn handle_newline(&mut self) -> TokenType {
         self.line += 1;
         TokenType::Skip
    }

    /// Peeks for a double quote or end of source,
    /// advancing in the process.
    ///
    /// If end is reached, prints unterminated string error.
    ///
    /// Advances past closing double quote, then returns the
    /// TokenType::String with collected value enclosed.
    fn handle_string(&mut self) -> TokenType {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { 
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error::unterminated_string(self.line);
            return TokenType::Skip;
        }
    
        // advance past the closing '"'
        self.advance();

        let str_val = self.source[self.start + 1.. self.current-1].to_string();
        TokenType::Literal(super::literal::Literal::String(str_val))

    }


    /// Peeks for more digits, advancing in the process.
    ///
    /// If a period is peeked, peeks next for another digit.
    /// If another is found, consumes the digit and all subsequent digits.
    ///
    /// Parses the final result as an f64 and returns a TokenType::Number
    /// enclosing it.
    fn handle_digit(&mut self) -> TokenType {
        
        // consume while digits
        while self.peek().is_digit(10) {
            self.advance();
        };

    
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the '.'
            self.advance();
            
            // consume trailing digits
            while self.peek().is_digit(10) {
                self.advance();
            };
        }

        match self.source[self.start + 1.. self.current-1].parse::<f64>() {
            Ok(val) => TokenType::Literal(super::literal::Literal::Number(val)),
            Err(e) => panic!("{e}")
        }
    }
    
    /// Advances while peek is alphanumeric.
    /// 
    /// matches result to a TokenType and returns it.
    fn handle_word(&mut self) -> TokenType {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let word = &self.source[self.start.. self.current];

        match word {
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
            _ => TokenType::Literal(super::literal::Literal::Identifier(word.to_string())),
        }

    }

    /// Advances and matches character to its simple token type 
    /// or handler. 
    ///
    /// Then retrieves a token result and adds it to self.tokens.
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
                    '!' => self.if_next_else('=', TokenType::BangEqual, TokenType::Bang),
                    '=' => self.if_next_else('=', TokenType::EqualEqual, TokenType::Equal),
                    '<' => self.if_next_else('=', TokenType::LessEqual, TokenType::Less),
                    '>' => self.if_next_else('=', TokenType::GreaterEqual, TokenType::Greater),
                    '/' => self.handle_slash(),
                    _ if c.is_digit(10) => self.handle_digit(),
                    _ if c.is_alphabetic() => self.handle_word(),
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

