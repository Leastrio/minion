use core::fmt;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Token {
  ILLEGAL,
  EOF,

  IDENT(String),
  INT(isize),

  ASSIGN,
  PLUS,
  MINUS,
  BANG,
  ASTERISK,
  SLASH,

  LT,
  GT,
  EQ,
  NOTEQ,

  COMMA,
  SEMICOLON,

  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,

  FUNCTION,
  LET,
  TRUE,
  FALSE,
  IF,
  ELSE,
  RETURN,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Token::IDENT(ident) => write!(f, "{}", ident),
      Token::INT(_) => write!(f, "INTEGER"),
      _ => write!(f, "{:?}", self),
    }
  }
}
