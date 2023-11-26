use std::{env, fs::{self, OpenOptions, File}, path::Path, io::{Write, BufRead}};

use rev_buf_reader::RevBufReader;

pub fn help() {
    println!("usage:");
    println!("check");
    println!("  Check kairn installation");

    println!("todo");
    println!("  Do something");

    println!("");
    println!("  Do something");

    println!("jot");
    println!("  Do something");
    println!("help");
    println!("  Display help text");
}

fn get_home_folder() -> String {
    let key = "HOME";
    let val = env::var(key).expect("");
    val
}

const DATA_FOLDER: &str = ".kairn";
const JOURNAL_LOG: &str = "journal.log";
const TODOS_FOLDER: &str = "todos";
const APP_NAME: &str = "kairn";


fn global_data_folder() -> String {
  format!("{}/{}", get_home_folder(), DATA_FOLDER)
}

fn global_journal() -> String {
  format!("{}/{}", global_data_folder(), JOURNAL_LOG)
}

fn global_todos() -> String {
  format!("{}/{}", global_data_folder(), TODOS_FOLDER)
}


fn check_path(str: String) -> bool {
  Path::new(str.as_str()).exists()
}

fn check_for_global_data_folder() -> bool {
    check_path(global_data_folder())
}

fn jot(blob: String) {
  let file = File::options()
    .append(true)
    .open(global_journal());
  
  match file {
    Ok(mut handle) => {
      let msg = format!("{}\n", blob);
      let write_op = handle.write_all(msg.as_bytes());
      if write_op.is_err() {
        println!("Could not append to journal");
      }
    },
    Err(_) => {}
  }
}

fn journal() {
  let file = File::options()
    .read(true)
    .open(global_journal())
    .expect("Journal file does not exist");

  let buf = RevBufReader::new(file);
  let limit = 10;
  let stuff: Vec<String> = buf.lines().take(limit).map(|l| l.expect("Could not parse line")).collect();
  for i in stuff.iter().rev() {
    println!("{}", i);
  }
}

fn check() {
  let global_data_status = check_for_global_data_folder();
  if !global_data_status {
    print!("Your kairn root data folder is missing");
    return;
  }
  if !check_path(global_journal()) {
    print!("Your kairn root journal is missing");
  }
  if !check_path(global_todos()) {
    print!("Your kairn root todos are missing");
  }
}

pub fn init() {
  if !check_for_global_data_folder() {
    match fs::create_dir(global_data_folder()) {
        Err(e) => {
          println!("Could not create root data folder because of an error: {}", e);
        },
        Ok(_) => {
          if !check_path(global_journal()) {
            let file = OpenOptions::new()
                        .read(true)
                        .write(true) // <--------- this
                        .create(true)
                        .open(global_journal());
            match file {
              Ok(_) => {},
              Err(e) => {
                println!("Could not create root journal log because of an error: {}", e);
              }
            }
          }
          if !check_path(global_todos()) {
            match fs::create_dir(global_todos()) {
              Ok(_) => {},
              Err(e) => {
                println!("Could not create root todos folder because of an error: {}", e);
              }
            }
          }
        }
    }
  }
}

pub fn home() {
    println!("hi");
    println!("welcome to {}!", APP_NAME);
}

pub fn handle_command(tokens: &[String]) {
  let command = &tokens[0];
  let rest = tokens[1..].iter().fold(String::new(), |a, b| a + b + " ");
  match command.as_str() {
    "help"  => help(),
    "check" => check(),
    "home"  => home(),
    "init"  => init(),
    "jot"   => jot(rest),
    "journal" => journal(),
    _       => {}
  }
}
