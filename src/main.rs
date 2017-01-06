//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

extern crate getopts;
extern crate itertools;
#[macro_use]
extern crate lazy_static;

use getopts::Options;
use std::env;
use std::io::{self, Write, BufRead};

mod lexer;
mod parser;
mod executor;
mod context;

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

  let mut current_result = 0i64;
  let mut ctx = context::Context::new();

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

    let result = executor::execute(&mut ctx, &ast.ok().unwrap());
    println!("the actual result: {:?}", result);

    match result {
      Ok(result) => {
        let res_name = format!("res{}", current_result);
        println!("{} = {} (hex: {:x} oct: {:o} bin: {:b})", res_name, result, result, result, result);
        ctx.add_variable(res_name, result);
        current_result += 1;
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

