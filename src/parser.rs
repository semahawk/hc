//
// parser.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use lexer::*;
use std::slice::Iter;
use std::iter::Peekable;
use itertools::Itertools;

#[derive(Debug)]
pub enum Expr {
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Assign(Box<Expr>, Box<Expr>),
  Ident(String),
  Number(u32),
}

macro_rules! generate_binop {
  ($high_fname:ident before $func_name:ident, $op1:ident -> $expr1:ident, $op2:ident -> $expr2:ident) => (
    fn $func_name(mut tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
      let mut lhs = $high_fname(&mut tokens);

      if lhs.is_none() {
        return None;
      }

      while let Some(op) = tokens.peeking_take_while(|t| match t { &&Token::$op1 => true, &&Token::$op2 => true, _ => false }).next() {
        let rhs = $high_fname(&mut tokens);

        if rhs.is_none() {
          return None;
        }

        lhs = match op {
          &Token::$op1 => {
            Some(Expr::$expr1(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())))
          },
          &Token::$op2 => {
            Some(Expr::$expr2(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())))
          },
          _ => {
            None
          },
        };
      };

      lhs
    }
  )
}

fn primary(tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
  if tokens.peek().is_none() {
    return None;
  }

  {
    let token = tokens.peek().unwrap();

    match token {
      &&Token::Integer(_) | &&Token::Ident(_) => (),
      _ => return None,
    };
  }

  let token = tokens.next().unwrap();

  match token {
    &Token::Integer(i) => Some(Expr::Number(i)),
    &Token::Ident(ref i) => Some(Expr::Ident(i.clone())),
    _ => None,
  }
}

generate_binop!(primary before mul, Star -> Mul, Slash -> Div);
generate_binop!(mul before add, Plus -> Add, Minus -> Sub);

fn assign(mut tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
  let mut lhs = add(&mut tokens);

  if lhs.is_none() {
    return None;
  }

  while let Some(op) = tokens.peeking_take_while(|t| match t { &&Token::Eq => true, _ => false }).next() {
    let rhs = add(&mut tokens);

    if rhs.is_none() {
      return None;
    }

    lhs = match op {
      &Token::Eq => {
        Some(Expr::Assign(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())))
      },
      _ => {
        None
      },
    };
  };

  lhs
}

fn expr(mut tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
  match assign(&mut tokens) {
    Some(expr) => Some(expr),
    None => None,
  }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Expr, &'static str> {
  let mut iter = tokens.iter().peekable();

  match expr(&mut iter) {
    Some(ast) => {
      Ok(ast)
    },
    None => {
      Err("Could not parse the tokens!")
    }
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

