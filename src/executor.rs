//
// executor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use parser::*;

#[derive(Debug)]
pub enum Value {
  Number(u32),
}

pub fn execute(expr: &Expr) -> Result<Value, &'static str> {
  match expr {
    &Expr::Number(i) => {
      Ok(Value::Number(i))
    },
    &Expr::Add(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs + rhs))
        }
      }
    },
    &Expr::Sub(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs - rhs))
        }
      }
    },
    &Expr::Mul(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs * rhs))
        }
      }
    },
    &Expr::Div(ref l, ref r) => {
      let lhs = execute(&*l).ok().unwrap();
      let rhs = execute(&*r).ok().unwrap();

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          if rhs == 0 {
            return Err("Division by zero!");
          }

          Ok(Value::Number(lhs / rhs))
        }
      }
    },
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

