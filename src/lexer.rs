use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
  input: &'a str,
  pos: usize,
  next_pos: usize,
  ch: u8,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut lexer = Lexer {
      input,
      pos: 0,
      next_pos: 0,
      ch: 0,
    };

    lexer.read_char();
    return lexer;
  }

  fn read_char(&mut self) {
    self.ch = if self.next_pos >= self.input.len() {
      0
    } else {
      self.input.as_bytes()[self.next_pos]
    };
    self.pos = self.next_pos;
    self.next_pos += 1;
  }

  fn peek_char(&mut self) -> u8 {
    if self.next_pos >= self.input.len() {
      return 0;
    } else {
      return self.input.as_bytes()[self.next_pos];
    }
  }

  fn skip_whitespace(&mut self) {
    while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
      self.read_char();
    }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    let token = match self.ch {
      b'=' => {
        if self.peek_char() == b'=' {
          self.read_char();
          Token::EQ
        } else {
          Token::ASSIGN
        }
      }
      b';' => Token::SEMICOLON,
      b'(' => Token::LPAREN,
      b')' => Token::RPAREN,
      b',' => Token::COMMA,
      b'+' => Token::PLUS,
      b'-' => Token::MINUS,
      b'!' => {
        if self.peek_char() == b'=' {
          self.read_char();
          Token::NOTEQ
        } else {
          Token::BANG
        }
      }
      b'*' => Token::ASTERISK,
      b'/' => Token::SLASH,
      b'<' => Token::LT,
      b'>' => Token::GT,
      b'}' => Token::RBRACE,
      b'{' => Token::LBRACE,
      ch if ch.is_ascii_alphabetic() || ch == b'_' => return self.read_identifier(),
      ch if ch.is_ascii_digit() => return self.read_number(),
      0 => Token::EOF,
      _ => Token::ILLEGAL,
    };

    self.read_char();
    return token;
  }

  fn read_identifier(&mut self) -> Token {
    let start = self.pos;
    while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
      self.read_char();
    }

    let identifier = &self.input[start..self.pos];

    match identifier {
      "fn" => Token::FUNCTION,
      "let" => Token::LET,
      "true" => Token::TRUE,
      "false" => Token::FALSE,
      "if" => Token::IF,
      "else" => Token::ELSE,
      "return" => Token::RETURN,
      _ => Token::IDENT(identifier.to_string()),
    }
  }

  fn read_number(&mut self) -> Token {
    let start = self.pos;
    while self.ch.is_ascii_digit() {
      self.read_char();
    }

    let digit = &self.input[start..self.pos];
    return Token::INT(digit.parse::<isize>().unwrap());
  }
}
