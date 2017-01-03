//
// lexer.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Token {
  Integer(u32),
  Ident(String),
  Plus,
  Minus,
  Star,
  Slash,
}

pub fn tokenize(input: &str) -> Vec<Token> {
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

      let mut value = c.to_digit(base).unwrap();

      while let Some(p) = iter.take_while_ref(|x| x.is_alphanumeric()).next() {
        value *= base;
        value += p.to_digit(base).unwrap();
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
      _ => (),
    }
  }

  return tokens;
}

/*
 * vi: ts=2:sw=2 expandtab
 */

