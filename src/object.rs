use core::fmt;

use crate::ast::{BlockStatement, Expression};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
  Integer(isize),
  Boolean(bool),
  Function(Vec<Expression>, BlockStatement),
  NoOp,
}

impl Object {
  pub fn is_truthy(&self) -> bool {
    match self {
      Object::Boolean(false) => false,
      _ => true,
    }
  }
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Integer(num) => write!(f, "{}", num),
      Object::Boolean(val) => write!(f, "{}", val),
      Object::Function(_params, _body) => write!(f, "[Function Object]"),
      Object::NoOp => write!(f, "NoOp"),
    }
  }
}

pub enum Interrupt {
  Return(Object),
  Error(String),
}

impl Interrupt {
  pub fn error(msg: String) -> EvalResult {
    return Err(Interrupt::Error(msg));
  }
}

pub type EvalResult = Result<Object, Interrupt>;
