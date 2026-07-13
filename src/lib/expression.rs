use crate::lib::{literal::Literal, token::Token};

// pub enum Literal {
//     True,
//     False,
//     Nil,
// }

pub enum Expr {
    Equality,
    Literal(Literal)
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
