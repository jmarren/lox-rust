use crate::lib::{expression::{Expr, Literal}, token::{Token, TokenType}};
use std::mem::discriminant;



pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser { 
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

  // private Expr equality() {
  //   Expr expr = comparison();
  //
  //   while (match(BANG_EQUAL, EQUAL_EQUAL)) {
  //     Token operator = previous();
  //     Expr right = comparison();
  //     expr = new Expr.Binary(expr, operator, right);
  //   }
  //
  //   return expr;
  // }
  //
   fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
    
        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison().clone();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        expr
   }

  // private Expr comparison() {
  //   Expr expr = term();
  //
  //   while (match(GREATER, GREATER_EQUAL, LESS, LESS_EQUAL)) {
  //     Token operator = previous();
  //     Expr right = term();
  //     expr = new Expr.Binary(expr, operator, right);
  //   }
  //
  //   return expr;
  // }
  //

  fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term().clone();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        expr
  }
  
  
  // private Expr term() {
    // Expr expr = factor();
    //
    // while (match(MINUS, PLUS)) {
    //   Token operator = previous();
    //   Expr right = factor();
    //   expr = new Expr.Binary(expr, operator, right);
    // }
  //   return expr;
  // }
    
   fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        
        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor().clone();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        
        expr
   }

  // private Expr factor() {
  //   Expr expr = unary();
  //
  //   while (match(SLASH, STAR)) {
  //     Token operator = previous();
  //     Expr right = unary();
  //     expr = new Expr.Binary(expr, operator, right);
  //   }
  //
  //   return expr;
  // }
    
   fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary().clone();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
    
        expr
   }

  // private Expr unary() {
  //   if (match(BANG, MINUS)) {
  //     Token operator = previous();
  //     Expr right = unary();
  //     return new Expr.Unary(operator, right);
  //   }
  //
  //   return primary();
  // }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary().clone();
            return Expr::Unary(operator, Box::new(right));
        }
        return self.primary();
    }

   fn expression(&mut self) -> Expr {
        self.equality()
   }

  // private Token consume(TokenType type, String message) {
  //   if (check(type)) return advance();
  //
  //   throw error(peek(), message);
  // }

   fn consume(&mut self, typ: &TokenType, message: &str) -> &Token {
        if self.check(typ) {
            return self.advance();    
        }
        
        panic!("peek = {:?}, message = {}", self.peek(), message);
   }


  // private Expr primary() {
  //   if (match(FALSE)) return new Expr.Literal(false);
  //   if (match(TRUE)) return new Expr.Literal(true);
  //   if (match(NIL)) return new Expr.Literal(null);
  //
  //   if (match(NUMBER, STRING)) {
  //     return new Expr.Literal(previous().literal);
  //   }
  //
  //   if (match(LEFT_PAREN)) {
  //     Expr expr = expression();
  //     consume(RIGHT_PAREN, "Expect ')' after expression.");
  //     return new Expr.Grouping(expr);
  //   }
  // }
  //  private Expr expression() {
  //   return equality();
  // }


    
   fn primary(&mut self) -> Expr {
        

        if self.match_tokens(vec![TokenType::False]) {
            return Expr::Literal(Literal::False);
        } 
        if self.match_tokens(vec![TokenType::True]) {
            return Expr::Literal(Literal::True);
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        // if we match to a Number then a String
        // return a literal 
        if self.match_tokens(vec![TokenType::Number(0.0), TokenType::String(String::from(""))]) {
            println!("matched number or string");
            return match &self.previous().unwrap().token_type {
                TokenType::String(s) => Expr::Literal(Literal::String(s.clone())),
                TokenType::Number(n) => Expr::Literal(Literal::Number(n.clone())),
                TokenType::Identifier(s) => Expr::Literal(Literal::Identifier(s.clone())),
                _ => panic!("previous is not String, Number, or Identifier"),
            };
        }

        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen, "Expect ')' after expression");
            return Expr::Grouping(Box::new(expr));
        }

        panic!("parser.primary failed to find match for {:?}", self.peek());
   }

   fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        // not at end so we can unwrap
        self.previous().unwrap()
   }

   
   fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
       println!("checking for {:?}", token_types);
       for t in token_types.iter() {
            println!("checking {:?}", t);
            if self.check(&t) {
                self.advance();
                return true;
            }
       }
       false
   }

   fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        discriminant(&(self.peek().unwrap().token_type)) == discriminant(token_type)
   }

   fn previous(&self) -> Option<&Token> { 
        self.tokens.iter().nth(self.current - 1)
   }
    
   fn is_at_end(&self) -> bool {
        self.peek().map_or(false, | x | matches!(x.token_type, TokenType::Eof))
   }


   fn peek(&self) -> Option<&Token> {
        self.tokens.iter().nth(self.current)
   }

 // private void synchronize() {
 //    advance();
 //
 //    while (!isAtEnd()) {
 //      if (previous().type == SEMICOLON) return;
 //
 //      switch (peek().type) {
 //        case CLASS:
 //        case FUN:
 //        case VAR:
 //        case FOR:
 //        case IF:
 //        case WHILE:
 //        case PRINT:
 //        case RETURN:
 //          return;
 //      }
 //
 //      advance();
 //    }
 //  }
 //
    
   fn sychronize(&mut self)  { 
        self.advance();

        while !self.is_at_end() {
            if self.previous().unwrap().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                TokenType::Return => {
                    return;
                },
                _ => {
                    self.advance();
                }
            }; 

        }
   }

    
}


  // private boolean match(TokenType... types) {
  //   for (TokenType type : types) {
  //     if (check(type)) {
  //       advance();
  //       return true;
  //     }
  //   }
  //
  //   return false;
  // }


