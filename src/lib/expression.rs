use crate::lib::{token::Token};

#[derive(Clone, Debug)]
pub enum Literal {
    True,
    False,
    Nil,
    String(String),
    Number(f64),
    Identifier(String),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Equality,
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}




// pub struct Expr {}

  // defineAst(outputDir, "Expr", Arrays.asList(
  //     "Binary   : Expr left, Token operator, Expr right",
  //     "Grouping : Expr expression",
  //     "Literal  : Object value",
  //     "Unary    : Token operator, Expr right"
  //   ));
  //

macro_derive::to_struct!();

macro_derive::expression!(Ident age i32 name String location String);

macro_derive::expression!(Binary left Expr operator Token right Expr);
// to_struct!();
