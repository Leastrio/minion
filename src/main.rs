mod ast;
mod env;
mod eval;
mod lexer;
mod object;
mod parser;
mod token;

use eval::Evaluator;
use lexer::Lexer;
use object::Interrupt;
use parser::Parser;

fn main() {
  let input = "
   let counter = fn(x) {
if (x > 100) {
return x;
} else {
let foobar = 9999;
counter(x + 1);
}
};
count(0); 
    ";

  let mut parser = Parser::new(Lexer::new(input));
  let program = parser.parse_program();

  if parser.errors.len() > 0 {
    for err in parser.errors {
      println!("{}", err);
    }
  } else {
    let mut eval = Evaluator::new();
    match eval.eval(program) {
      Ok(obj) | Err(Interrupt::Return(obj)) => println!("Program returned: {}", obj),
      Err(Interrupt::Error(err)) => println!("{}", err),
    }
  }
}
