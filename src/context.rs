//
// context.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use std::collections::HashMap;

use executor::*;

#[derive(Debug)]
pub struct Context {
  vars: HashMap<String, Value>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      vars: HashMap::new(),
    }
  }

  pub fn add_variable(&mut self, varname: String, value: Value) {
    self.vars.insert(varname, value);
  }

  pub fn lookup_variable(&self, varname: String) -> Option<Value> {
    match self.vars.get(&*varname) {
      Some(value) => Some(*value),
      None => None,
    }
  }
}

/*
 * vi: sw=2:ts=2 expandtab
 */

