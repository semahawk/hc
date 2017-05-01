//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

extern crate getopts;
extern crate itertools;
extern crate rustyline;

use getopts::Options;
use std::env;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lexer;
mod parser;
mod executor;
mod context;

const LOG_10_2: f64 = 0.301029995663981195213738894724493026768189881462108541310;

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

  let mut rl = Editor::<()>::new();

  let mut current_result = 0i64;
  let mut ctx = context::Context::new();

  ctx.add_variable(String::from("zeropad"), executor::Value::Number(32));

  loop {
    let readline = rl.readline(">> ");

    let mut input = match readline {
      Ok(input) => {
        rl.add_history_entry(&input);
        input
      },
      Err(ReadlineError::Interrupted) => {
        break
      },
      Err(ReadlineError::Eof) => {
        break
      }
      Err(err) => {
        println!("error: {:?}", err);
        break
      }
    };

    let tokens = match lexer::tokenize(&input) {
      Ok(tokens) => tokens,
      Err(err) => {
        println!("error: {}", err);
        input.clear();
        continue
      },
    };

    let ast = match parser::parse(&tokens) {
      Ok(ast) => ast,
      Err(err) => {
        println!("error: {}", err);
        input.clear();
        continue
      },
    };

    match executor::execute(&mut ctx, &ast) {
      Ok(result) => {
        let res_name = format!("res{}", current_result);
        let zero_pad_bits = match ctx.lookup_variable(String::from("zeropad")) {
          Some(zeropad) => match zeropad {
            executor::Value::Number(zeropad) => zeropad,
          },
          None => 32,
        };

        println!("{} = {:0d_width$} (hex: {:0x_width$x} bin: {:0b_width$b}; {})",
          res_name, result, result, result, result.to_nice_unit(),
          d_width = (LOG_10_2 * (zero_pad_bits as f64)).ceil() as usize,
          x_width = (zero_pad_bits as usize) / 4,
          b_width = (zero_pad_bits as usize));

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

