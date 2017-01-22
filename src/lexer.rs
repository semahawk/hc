//
// lexer.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Token {
  Integer(i64),
  Ident(String),
  Plus,
  Minus,
  Star,
  Slash,
  Eq,
  And,
  Pipe,
  Caret,
  Lt,
  DoubleLt,
  Gt,
  DoubleGt,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
  let mut tokens = Vec::new();

  let mut iter = input.chars().peekable();

  while let Some(c) = iter.next() {
    if c.is_whitespace() {
      continue;
    }

    if c.is_numeric() {
      let mut base = 10;

      if c == '0' {
        match iter.peek() {
          Some(&'x') => { iter.next(); base = 16 },
          Some(&'c') => { iter.next(); base =  8 },
          Some(&'q') => { iter.next(); base =  4 },
          Some(&'b') => { iter.next(); base =  2 },
          Some(&_) =>   { iter.next(); base = 10 },
          None => (),
        }
      }

      let mut value: i64 = c.to_digit(base).unwrap() as i64;

      while let Some(p) = iter.take_while_ref(|x| x.is_alphanumeric()).next() {
        value *= base as i64;
        value += p.to_digit(base).unwrap() as i64;
      }

      tokens.push(Token::Integer(value));
      continue;
    }

    if c.is_alphanumeric() {
      let mut name = String::new();

      name.push(c);
      while let Some(p) = iter.take_while_ref(|x| x.is_alphanumeric()).next() {
        name.push(p);
      }

      tokens.push(Token::Ident(name));
      continue;
    }

    match c {
      '+' => tokens.push(Token::Plus),
      '-' => tokens.push(Token::Minus),
      '*' => tokens.push(Token::Star),
      '/' => tokens.push(Token::Slash),
      '=' => tokens.push(Token::Eq),
      '&' => tokens.push(Token::And),
      '|' => tokens.push(Token::Pipe),
      '^' => tokens.push(Token::Caret),
      '<' => {
        match iter.peek() {
          Some(&'<') => { iter.next(); tokens.push(Token::DoubleLt); },
          _ => tokens.push(Token::Lt),
        }
      },
      '>' => {
        match iter.peek() {
          Some(&'>') => { iter.next(); tokens.push(Token::DoubleGt); },
          _ => tokens.push(Token::Gt),
        }
      },
      ch  => return Err(format!("Unknown character '{}'", ch)),
    }
  }

  return Ok(tokens);
}

/*
 * vi: ts=2:sw=2 expandtab
 */

