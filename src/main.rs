use std::env;
mod kairn;

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => {
      kairn::home();
    },
    _ => {
      kairn::help();
    }
  }
}
