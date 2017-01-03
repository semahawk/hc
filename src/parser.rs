//
// parser.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use lexer::*;
use std::slice::Iter;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expr {
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Number(u32),
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
    &Token::Ident(_) => Some(Expr::Number(1337)),
    _ => None,
  }
}

fn add(mut tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
  let lhs = primary(&mut tokens);

  if lhs.is_none() {
    return None;
  }

  if let Some(op) = tokens.peek() {
    match op {
      &&Token::Plus | &&Token::Minus => (),
      _ => return lhs,
    }
  } else {
    return lhs;
  }

  let op = tokens.next().unwrap();
  let rhs = expr(&mut tokens);

  if rhs.is_none() {
    return None;
  }

  match op {
    &Token::Plus => {
      Some(Expr::Add(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())))
    },
    &Token::Minus => {
      Some(Expr::Sub(Box::new(lhs.unwrap()), Box::new(rhs.unwrap())))
    },
    _ => {
      None
    },
  }
}

fn expr(mut tokens: &mut Peekable<Iter<Token>>) -> Option<Expr> {
  match add(&mut tokens) {
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

