use crate::{
  ast::{self, BlockStatement, Expression, Precedence, Program, Statement},
  lexer::Lexer,
  token::Token,
};
use core::fmt;

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  curr_token: Token,
  peek_token: Token,
  pub errors: Vec<ParserError>,
}

#[derive(Debug)]
pub enum ParserError {
  // expected, got
  UnexpectedToken(Token, Token),
  UnknownPrefix(Token),
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParserError::UnexpectedToken(expected, got) => {
        write!(f, "Expected: {}, got: {}", expected, got)
      }
      ParserError::UnknownPrefix(got) => write!(f, "Unknown prefix, got: {}", got),
    }
  }
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let curr_token = lexer.next_token();
    let peek_token = lexer.next_token();
    Parser {
      lexer,
      curr_token,
      peek_token,
      errors: vec![],
    }
  }

  fn next_token(&mut self) {
    self.curr_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program: Program = vec![];

    while self.curr_token != Token::EOF {
      if let Some(statement) = self.parse_statement() {
        program.push(statement);
      }
      self.next_token();
    }

    return program;
  }

  fn parse_statement(&mut self) -> Option<Statement> {
    match self.curr_token {
      Token::LET => self.parse_let_statement(),
      Token::RETURN => self.parse_return_statement(),
      _ => self.parse_expression_statement(),
    }
  }

  fn parse_expression_statement(&mut self) -> Option<Statement> {
    let parse_expression = self.parse_expression(Precedence::LOWEST);
    let statement = Statement::Expression(parse_expression?);
    if self.peek_token == Token::SEMICOLON {
      self.next_token();
    }

    return Some(statement);
  }

  fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
    let mut left = match self.curr_token {
      Token::IDENT(_) => Some(Expression::Ident(self.curr_token.clone())),
      Token::INT(_) => Some(Expression::Integer(self.curr_token.clone())),
      Token::BANG | Token::MINUS => self.parse_prefix_expression(),
      Token::TRUE | Token::FALSE => Some(Expression::Boolean(self.curr_token.clone())),
      Token::LPAREN => self.parse_grouped_expression(),
      Token::IF => self.parse_if_expression(),
      Token::FUNCTION => self.parse_function_expression(),
      _ => {
        self.prefix_error(self.curr_token.clone());
        return None;
      }
    };

    while self.peek_token != Token::SEMICOLON && precedence < ast::get_precedence(&self.peek_token)
    {
      left = match self.peek_token {
        Token::PLUS
        | Token::MINUS
        | Token::SLASH
        | Token::ASTERISK
        | Token::EQ
        | Token::NOTEQ
        | Token::LT
        | Token::GT => {
          self.next_token();
          self.parse_infix_expression(left?)
        }
        Token::LPAREN if valid_function_identifier(&left) => {
          self.next_token();
          self.parse_call_expression(left?)
        }
        _ => {
          return left;
        }
      };
    }

    return left;
  }

  fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
    let arguments = self.parse_call_args()?;
    return Some(Expression::Call(Box::new(function), arguments));
  }

  fn parse_call_args(&mut self) -> Option<Vec<Expression>> {
    let mut args: Vec<Expression> = vec![];
    if self.peek_token == Token::RPAREN {
      self.next_token();
      return Some(args);
    }

    self.next_token();
    args.push(self.parse_expression(Precedence::LOWEST)?);

    while self.peek_token == Token::COMMA {
      self.next_token();
      self.next_token();
      args.push(self.parse_expression(Precedence::LOWEST)?);
    }

    if !self.expect_peek_token(Token::RPAREN) {
      return None;
    }

    return Some(args);
  }

  fn parse_function_expression(&mut self) -> Option<Expression> {
    if !self.expect_peek_token(Token::LPAREN) {
      return None;
    }

    let params = self.parse_function_params()?;

    if !self.expect_peek_token(Token::LBRACE) {
      return None;
    }

    let body = self.parse_block_statement();

    return Some(Expression::Function(params, body));
  }

  fn parse_function_params(&mut self) -> Option<Vec<Expression>> {
    let mut params: Vec<Expression> = vec![];

    if self.peek_token == Token::RPAREN {
      self.next_token();
      return Some(params);
    }

    self.next_token();

    params.push(Expression::Ident(self.curr_token.clone()));
    while self.peek_token == Token::COMMA {
      self.next_token();
      self.next_token();
      params.push(Expression::Ident(self.curr_token.clone()));
    }

    if !self.expect_peek_token(Token::RPAREN) {
      return None;
    }

    return Some(params);
  }

  fn parse_if_expression(&mut self) -> Option<Expression> {
    if !self.expect_peek_token(Token::LPAREN) {
      return None;
    }

    self.next_token();
    let condition = self.parse_expression(Precedence::LOWEST);

    if !self.expect_peek_token(Token::RPAREN) {
      return None;
    }
    if !self.expect_peek_token(Token::LBRACE) {
      return None;
    }

    let consequence = self.parse_block_statement();
    let mut alternative = None;

    if self.peek_token == Token::ELSE {
      self.next_token();
      if !self.expect_peek_token(Token::LBRACE) {
        return None;
      }

      alternative = Some(self.parse_block_statement());
    }

    return Some(Expression::If(
      Box::new(condition?),
      consequence,
      alternative,
    ));
  }

  fn parse_block_statement(&mut self) -> BlockStatement {
    let mut block: BlockStatement = vec![];
    self.next_token();
    while self.curr_token != Token::RBRACE && self.curr_token != Token::EOF {
      if let Some(statement) = self.parse_statement() {
        block.push(statement);
      }
      self.next_token();
    }

    return block;
  }

  fn parse_grouped_expression(&mut self) -> Option<Expression> {
    self.next_token();
    let expression = self.parse_expression(Precedence::LOWEST);

    if !self.expect_peek_token(Token::RPAREN) {
      return None;
    }

    return expression;
  }

  fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
    let curr = self.curr_token.clone();
    let curr_precedence = ast::get_precedence(&curr);
    self.next_token();
    return Some(Expression::Infix(
      curr,
      Box::new(left),
      Box::new(self.parse_expression(curr_precedence)?),
    ));
  }

  fn parse_prefix_expression(&mut self) -> Option<Expression> {
    let curr = self.curr_token.clone();
    self.next_token();
    return Some(Expression::Prefix(
      curr,
      Box::new(self.parse_expression(Precedence::PREFIX)?),
    ));
  }

  fn parse_return_statement(&mut self) -> Option<Statement> {
    self.next_token();
    let expression = self.parse_expression(Precedence::LOWEST);
    if self.peek_token == Token::SEMICOLON {
      self.next_token();
    }
    return Some(Statement::Return(expression?));
  }

  fn parse_let_statement(&mut self) -> Option<Statement> {
    if !self.expect_peek_token(Token::IDENT(String::new())) {
      return None;
    }

    let ident = self.curr_token.clone();

    if !self.expect_peek_token(Token::ASSIGN) {
      return None;
    }

    self.next_token();
    let expression = self.parse_expression(Precedence::LOWEST);

    if self.peek_token == Token::SEMICOLON {
      self.next_token();
    }

    return Some(Statement::Let(ident, expression?));
  }

  fn expect_peek_token(&mut self, token: Token) -> bool {
    if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&token) {
      self.next_token();
      return true;
    } else {
      self.peek_error(token);
      return false;
    }
  }

  fn peek_error(&mut self, token: Token) {
    self
      .errors
      .push(ParserError::UnexpectedToken(token, self.peek_token.clone()))
  }

  fn prefix_error(&mut self, token: Token) {
    self.errors.push(ParserError::UnknownPrefix(token))
  }
}

fn valid_function_identifier(left: &Option<Expression>) -> bool {
  match left {
    Some(Expression::Ident(_)) => true,
    Some(Expression::Function(_, _)) => true,
    _ => false,
  }
}
