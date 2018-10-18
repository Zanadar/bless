use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str;
extern crate colored;

use colored::*;

fn main() -> std::io::Result<()> {
  let tty = fs::OpenOptions::new()
    .read(true)
    .write(true)
    .open("/dev/tty")?;

  let mut tty = BufReader::new(tty);
  let mut piped = String::new();
  io::stdin().read_to_string(&mut piped)?;

  println!("piped:\n{}", piped);
  let mut buf = Vec::<u8>::new();

  loop {
    let num_bytes = tty.read_until(b'\n', &mut buf)?;
    if buf.starts_with(&[27]) {
      println!("Escape!\n{:?}", buf);
      buf.clear();
      continue;
    }
    if num_bytes == 0 {
      break;
    }
    buf.pop();
    println!("{}", str::from_utf8(&buf).unwrap().blue());
    buf.clear();
  }

  Ok(())
}
