

#[derive(Debug, PartialEq)]
pub enum Literal {
  Identifier(String),
  String(String),
  Number(f64),
}

