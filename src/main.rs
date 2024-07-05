mod ast;
mod env;
mod eval;
mod lexer;
mod minions;
mod object;
mod parser;
mod token;

use std::io::{self, Write};

use eval::Evaluator;
use lexer::Lexer;
use object::Interrupt;
use parser::Parser;

fn main() {
  let mut eval = Evaluator::new();

  println!("Welcome to the Minion REPL 🍌🍌🍌🍌");
  println!("-----------------------------------");

  loop {
    let mut input = String::new();
    print!(">>> ");
    io::stdout().flush().unwrap();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read repl line");

    let mut parser = Parser::new(Lexer::new(&input));
    let program = parser.parse_program();

    if parser.errors.len() > 0 {
      // TODO: Add support for block expressions
      for error in parser.errors {
        match error {
          parser::ParserError::UnexpectedToken(_, _) => println!("{}", error),
          parser::ParserError::UnknownPrefix(_) => println!("{}", error),
          parser::ParserError::InvalidIdent(_) => println!("{}", error),
        }
      }
    } else {
      match eval.eval(program) {
        Ok(object::Object::NoOp) => (),
        Ok(obj) | Err(Interrupt::Return(obj)) => println!("{}", obj),
        Err(Interrupt::Error(err)) => println!("{}", err),
      }
    }
  }
}
