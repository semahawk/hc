//
// executor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use std::fmt;

use parser::*;
use context::*;

#[derive(Debug, Clone, Copy)]
pub enum Value {
  Number(i64),
}

impl Value {
  pub fn to_nice_unit(&self) -> String {
    let mut units = vec!["EiB", "PiB", "TiB", "GiB", "MiB", "KiB", "B"];
    let mut final_unit = units.pop().unwrap();

    match self {
      &Value::Number(orig_value) => {
        let mut value = orig_value as f64;
        'a: while value >= 1024f64 {
          if let Some(unit) = units.pop() {
            final_unit = unit;
            value /= 1024f64;
          } else {
            break 'a;
          }
        }

        return format!("{} {}", value, final_unit);
      },
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Value::Number(n) => {
        let width = f.width().unwrap_or(0);
        write!(f, "{:width$}", n, width = width)
      },
    }
  }
}

impl fmt::Binary for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:b}", n),
    }
  }
}

impl fmt::Octal for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:o}", n),
    }
  }
}

impl fmt::LowerHex for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    match self {
      &Value::Number(n) => write!(f, "{:x}", n),
    }
  }
}

pub fn execute(ctx: &mut Context, expr: &Expr) -> Result<Value, String> {
  match expr {
    &Expr::Number(i) => {
      Ok(Value::Number(i))
    },
    &Expr::Ident(ref varname) => {
      match ctx.lookup_variable(varname.clone()) {
        Some(value) => Ok(value),
        None => Err(format!("Variable '{}' not found!", varname)),
      }
    },
    &Expr::Add(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs + rhs))
        }
      }
    },
    &Expr::Sub(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs - rhs))
        }
      }
    },
    &Expr::Mul(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs * rhs))
        }
      }
    },
    &Expr::Div(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          if rhs == 0 {
            return Err(String::from("Division by zero!"));
          }

          Ok(Value::Number(lhs / rhs))
        }
      }
    },
    &Expr::BitwiseAnd(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs & rhs))
        }
      }
    },
    &Expr::BitwiseShl(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs << rhs))
        }
      }
    },
    &Expr::BitwiseShr(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs >> rhs))
        }
      }
    },
    &Expr::BitwiseXor(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs ^ rhs))
        }
      }
    },
    &Expr::BitwiseOr(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs | rhs))
        }
      }
    },
    &Expr::BitwiseSetBit(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs | (1 << rhs)))
        }
      }
    },
    &Expr::BitwiseUnsetBit(ref l, ref r) => {
      let lhs = try!(execute(ctx, &*l));
      let rhs = try!(execute(ctx, &*r));

      match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => {
          Ok(Value::Number(lhs & !(1 << rhs)))
        }
      }
    },
    &Expr::Assign(ref l, ref r) => {
      let rhs = try!(execute(ctx, &*r));
      let ref lhs = **l;

      let varname = match lhs {
        &Expr::Ident(ref varname) => varname.clone(),
        _ => return Err(String::from("Can only assign to an identifier!")),
      };

      match rhs {
        Value::Number(value) => {
          ctx.add_variable(varname, rhs);

          Ok(Value::Number(value))
        }
      }
    },
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

