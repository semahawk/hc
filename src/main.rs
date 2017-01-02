extern crate getopts;
use getopts::Options;
use std::env;
use std::io::{self, Write, BufRead};

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

    println!("your input: {}", input);

    input.clear();
  }
}

/*
 * vi: sw=2:ts=2 expandtab
 */
