use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
  // Identifier, Expression
  Let(Token, Expression),
  Return(Expression),
  Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
  Ident(Token),
  Integer(Token),
  // Token, Right
  Prefix(Token, Box<Expression>),
  // Token, Left, Right
  Infix(Token, Box<Expression>, Box<Expression>),
  Boolean(Token),
  // Condition, Consequence, Alternative
  If(Box<Expression>, BlockStatement, Option<BlockStatement>),
  // params, body
  Function(Vec<Expression>, BlockStatement),
  // function / identifier, arguments
  Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST,
  EQUALS,
  LESSGREATER,
  SUM,
  PRODUCT,
  PREFIX,
  CALL,
}

pub fn get_precedence(token: &Token) -> Precedence {
  match token {
    Token::EQ | Token::NOTEQ => Precedence::EQUALS,
    Token::LT | Token::GT => Precedence::LESSGREATER,
    Token::PLUS | Token::MINUS => Precedence::SUM,
    Token::SLASH | Token::ASTERISK => Precedence::PRODUCT,
    Token::LPAREN => Precedence::CALL,
    _ => Precedence::LOWEST,
  }
}

pub type Program = BlockStatement;
pub type BlockStatement = Vec<Statement>;
