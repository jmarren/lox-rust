use crate::lib::{expression::{Expr, Literal }, literal::Literal, token::{Token, TokenType}};
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
   fn equality(&mut self) {
        let expr = Expr::Equality;
    
        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            
            // let right = 
        }

        // while 
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

  // fn comparison(
  
  
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
    
   // fn term(&self) -> Expr {
   //
   //
   // }

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
    
   // fn factor(&self) -> Expr {
   //
   // }

  // private Expr unary() {
  //   if (match(BANG, MINUS)) {
  //     Token operator = previous();
  //     Expr right = unary();
  //     return new Expr.Unary(operator, right);
  //   }
  //
  //   return primary();
  // }

    fn unary(&self) -> Expr {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr{};
        }
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
  
    
   fn primary(&self) -> Expr {
        if self.match_tokens(vec![TokenType::False]) {
            return Expr::Literal(Literal::False);
        } 
        if self.match_tokens(vec![TokenType::True]) {
            return Expr::Literal(Literal::True);
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }

        if self.match_tokens(vec![TokenType::Number(0), TokenType::String(String::from(""))]) {
            match self.previous().unwrap() {
                Literal::String()
            }
            return Expr::Literal(self.previous().unwrap())
        }

        
        
   }

   fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        // not at end so we can unwrap
        self.previous().unwrap()
   }

   
   fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
       for t in token_types.iter() {
            if self.check(t) {
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
        
        discriminant(&self.peek().unwrap().token_type) == discriminant(token_type)
   }

   fn previous(&self) -> Option<&Token> { 
        self.tokens.iter().nth(self.current - 1)
   }
    
   fn is_at_end(&self) -> bool {
        self.peek().map_or(false, | x | !matches!(x.token_type, TokenType::Eof))
   }


   fn peek(&self) -> Option<&Token> {
        self.tokens.iter().nth(self.current)
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
