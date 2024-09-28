use std::process::{Command, Output};
use colored::Colorize;

pub fn git (args: &[&str]) -> Output {
  let log = format!("+> git {}", args.join(" "));
  println!("{}", log.bright_black());

  Command::new("git")
    .args(args)
    .output()
    .expect("failed to run git command, make sure git is installed on your machine")
}

pub fn read_project_name () -> String {
  let output = git(&["remote", "get-url", "origin"]);
  let url = String::from_utf8_lossy(&output.stdout);
  let url = url.trim().to_string();
  url.split("/").last().unwrap().to_string()
}
