use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::env::Environment;
use crate::object::{EvalResult, Interrupt, Object};
use crate::token::Token;

pub struct Evaluator {
  pub env: Rc<RefCell<Environment>>,
}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {
      env: Rc::new(RefCell::new(Environment {
        store: HashMap::new(),
        outer: None,
      })),
    }
  }

  pub fn eval(&mut self, program: Program) -> EvalResult {
    let mut res: EvalResult = Ok(Object::NoOp);
    for statement in program {
      res = self.eval_statement(statement);

      if let Err(_) = res {
        return res;
      }
    }

    return res;
  }

  fn eval_block(&mut self, block: BlockStatement) -> EvalResult {
    let mut res: EvalResult = Ok(Object::NoOp);
    for statement in block {
      res = self.eval_statement(statement);
      if let Err(_) = res {
        return res;
      }
    }
    return res;
  }

  fn eval_statement(&mut self, statement: Statement) -> EvalResult {
    match statement {
      Statement::Let(Token::IDENT(var), expr) => {
        let res = self.eval_expression(expr)?;
        self.env.borrow_mut().insert(var, res);
        Ok(Object::NoOp)
      }
      Statement::Return(expr) => Err(Interrupt::Return(self.eval_expression(expr)?)),
      Statement::Expression(expr) => self.eval_expression(expr),
      _ => Ok(Object::NoOp),
    }
  }

  fn eval_expression(&mut self, expr: Expression) -> EvalResult {
    match expr {
      Expression::Ident(Token::IDENT(ident)) => match self.env.borrow().get(&ident) {
        Some(val) => Ok(val),
        None => Interrupt::error(format!("Invalid variable name {}", ident)),
      },
      Expression::Integer(Token::INT(num)) => Ok(Object::Integer(num)),
      Expression::Prefix(op, expr) => {
        let expr = self.eval_expression(*expr)?;
        self.eval_prefix_expression(op, expr)
      }
      Expression::Infix(op, left, right) => {
        let left = self.eval_expression(*left)?;
        let right = self.eval_expression(*right)?;
        self.eval_infix_expression(op, left, right)
      }
      Expression::Boolean(Token::TRUE) => Ok(Object::Boolean(true)),
      Expression::Boolean(Token::FALSE) => Ok(Object::Boolean(false)),
      Expression::If(cond, consq, alt) => self.eval_if_expression(cond, consq, alt),
      Expression::Function(params, body) => Ok(Object::Function(params, body)),
      Expression::Call(func, args) => self.eval_call_expression(func, args),
      _ => Interrupt::error("Invalid Expression".to_string()),
    }
  }

  fn eval_call_expression(&mut self, func: Box<Expression>, args: Vec<Expression>) -> EvalResult {
    if let Object::Function(params, body) = self.eval_expression(*func)? {
      let mut enclosed_env = Environment::new_with_outer(self.env.clone());
      for (param, arg) in params.iter().zip(args) {
        let param_name = match param {
          Expression::Ident(Token::IDENT(inner)) => inner,
          _ => return Interrupt::error("Invalid variable name in function params".to_string()),
        };
        enclosed_env.insert(param_name.to_string(), self.eval_expression(arg)?);
      }
      let mut eval = Evaluator {
        env: Rc::new(RefCell::new(enclosed_env)),
      };
      return eval.eval_block(body.clone());
    }

    return Interrupt::error("Invalid function name/expression getting called".to_string());
  }

  fn eval_if_expression(
    &mut self,
    condition: Box<Expression>,
    consequence: BlockStatement,
    alt: Option<BlockStatement>,
  ) -> EvalResult {
    let cond = self.eval_expression(*condition)?;
    if cond.is_truthy() {
      return self.eval_block(consequence);
    } else {
      if let Some(alternative) = alt {
        return self.eval_block(alternative);
      } else {
        return Ok(Object::NoOp);
      }
    }
  }

  fn eval_infix_expression(&mut self, op: Token, left: Object, right: Object) -> EvalResult {
    match op {
      Token::LT => Ok(Object::Boolean(left < right)),
      Token::GT => Ok(Object::Boolean(left > right)),
      Token::EQ => Ok(Object::Boolean(left == right)),
      Token::NOTEQ => Ok(Object::Boolean(left != right)),
      _ => {
        let left_int = match left {
          Object::Integer(num) => num,
          _ => return Interrupt::error(format!("Left side of expression is not an int {}", left)),
        };
        let right_int = match right {
          Object::Integer(num) => num,
          _ => {
            return Interrupt::error(format!("Right side of expression is not an int {}", right))
          }
        };

        match op {
          Token::PLUS => Ok(Object::Integer(left_int + right_int)),
          Token::MINUS => Ok(Object::Integer(left_int - right_int)),
          Token::ASTERISK => Ok(Object::Integer(left_int * right_int)),
          Token::SLASH => Ok(Object::Integer(left_int / right_int)),
          _ => Interrupt::error("Invalid infix operator!".to_string()),
        }
        // request_math_assistance(left, right, op)
      }
    }
  }

  #[allow(dead_code)]
  fn request_math_assistance(left: Object, right: Object, op: Token) -> EvalResult {
    let op = match op {
      Token::PLUS => "+",
      Token::MINUS => "-",
      Token::ASTERISK => "*",
      Token::SLASH => "/",
      _ => return Interrupt::error("Invalid math operator".to_string()),
    };

    let mut input = String::new();
    print!("What is: {} {} {}? ", left, op, right);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let answer = input.trim().parse::<isize>().unwrap();
    Ok(Object::Integer(answer))
  }

  fn eval_prefix_expression(&mut self, op: Token, right: Object) -> EvalResult {
    match op {
      Token::BANG => Ok(self.eval_bang_op(right)),
      Token::MINUS => self.eval_minus_op(right),
      _ => Interrupt::error("Invalid prefix operator".to_string()),
    }
  }

  fn eval_minus_op(&mut self, right: Object) -> EvalResult {
    match right {
      Object::Integer(num) => Ok(Object::Integer(-num)),
      _ => Interrupt::error("Right side of - operator is not a valid integer".to_string()),
    }
  }

  fn eval_bang_op(&mut self, right: Object) -> Object {
    match right {
      Object::Boolean(true) => Object::Boolean(false),
      Object::Boolean(false) => Object::Boolean(true),
      _ => Object::Boolean(false),
    }
  }
}
