use std::{fs::{remove_file, File}, io::Write};

pub fn init_project () {
  // Nothing to do !
}

pub fn deinit_project () {
  remove_file("src/testez.ts").unwrap();
}

pub fn use_boilerplate (boilerplate: &str, arguments: Vec<String>) {
  let mut content = boilerplate.to_string();

  for (index, argument) in arguments.iter().enumerate() {
    content = content.replace(&format!("[#{}#]", index), argument);
  }

  let mut file = File::create("src/testez.ts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

pub fn run_test () -> String {
  let output = std::process::Command::new("bun")
    .args(["run", "./src/testez.ts"])
    .output()
    .expect("failed to run bun command, make sure bun is installed on your machine");

  if !output.stderr.is_empty() {
    let error = String::from_utf8_lossy(&output.stderr);
    panic!("{}", error);
  }

  let output = String::from_utf8_lossy(&output.stdout);
  output.to_string()
}
