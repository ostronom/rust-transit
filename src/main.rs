#![feature(collections)]

use std::io::{Write, Result, stdout};
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;


pub enum Mode { AsKey, AsVal }

pub trait Transit {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize>;
}

impl Transit for u8 {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize> {
    self.to_string().write(w, mode)
  }
}

impl Transit for String {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize> {
    w.write(self.as_bytes())
  }
}

impl Transit for bool {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize> {
    let r = if *self { "true" } else { "false" };
    w.write(r.as_bytes())
  }
}

impl<T: Transit> Transit for [T] {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize> {
    self.iter().fold(Ok(0), |a, x| a.and(x.write(w, Mode::AsVal)))
  }
}

impl<K: Transit + Hash + Eq, V: Transit> Transit for HashMap<K, V> {
  fn write(&self, w: &mut Write, mode: Mode) -> Result<usize> {
    self.iter().fold(Ok(0), |a, (k,v)| {
      a.and(k.write(w, Mode::AsKey))
       .and(w.write(",".as_bytes()))
       .and(v.write(w, Mode::AsVal))
    })
  }
}

pub fn encode<T: Transit>(t: T) -> Result<usize> {
  let mut w = stdout(); // TODO: make this generic writer
  t.write(w.by_ref(), Mode::AsVal)
}

fn main() {
  //let s = String::from_str("TEST");
  let mut s = HashMap::new();
  s.insert(1, String::from_str("B"));
  encode(s);
}