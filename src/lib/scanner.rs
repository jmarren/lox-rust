use crate::lib::{error::{self, unexpected_character}, token::{Token, TokenType}};


fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}


fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || 
    (c >= 'A' && c <= 'Z') || 
    c == '_'
}


fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}



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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));
        &self.tokens
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
    fn peek(&self) -> char {
        match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => '\0',
        }
    }
    
    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(c) => c,
            None => '\0',
        }
    }
    // if we find a slash we check for another slash
    // if present, we consume tokens until the end of the line
    // and return Skip
    //
    // otherwise we treat it as a division operator and return Slash
    fn handle_slash(&mut self) -> TokenType {
         if self.try_match('/') {
            while self.peek() != '\n'  && !self.is_at_end() {
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
        TokenType::String(str_val)

    }


    
    fn handle_digit(&mut self) -> TokenType {
        
        // consume while digits
        while is_digit(self.peek()) {
            self.advance();
        };

    
        if self.peek() == '.' && is_digit(self.peek_next()) {
            // consume the '.'
            self.advance();
            
            // consume trailing digits
            while is_digit(self.peek()) {
                self.advance();
            };
        }
        match self.source[self.start + 1.. self.current-1].parse::<f64>() {
            Ok(val) => TokenType::Number(val),
            Err(e) => panic!("{e}")
        }
    }

    fn handle_word(&mut self) -> TokenType {
        while is_alphanumeric(self.peek()) {
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
            _ => TokenType::Identifier(word.to_string()),
        }

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
                    _ if is_digit(c) => self.handle_digit(),
                    _ if is_alpha(c) => self.handle_word(),
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

