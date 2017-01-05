//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

extern crate getopts;
extern crate itertools;

use getopts::Options;
use std::env;
use std::io::{self, Write, BufRead};

mod lexer;
mod parser;
mod executor;

fn print_usage(program: &str, opts: Options)
{
  let brief = format!("Usage: {} [options]", program);

  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }

  let stdio = io::stdin();
  let mut input = String::new();

  loop {
    print!("=> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    match stdio.lock().read_line(&mut input) {
      Err(err) => println!("# error: {}", err),
      Ok(_) => (),
    }

    let tokens = lexer::tokenize(&input);
    println!("your tokenized input: {:?}", tokens);

    let ast = parser::parse(&tokens);
    println!("the AST made from the tokens: {:?}", ast);

    let result = executor::execute(&ast.ok().unwrap());
    println!("the actual result: {:?}", result);

    match result {
      Ok(result) => {
        println!("res = {} (hex: {:x} oct: {:o} bin: {:b})", result, result, result, result);
      },
      Err(err) => {
        println!("error: {}", err);
      }
    }

    input.clear();
  }
}

/*
 * vi: sw=2:ts=2 expandtab
 */

