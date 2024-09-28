mod implementations;
use implementations::kotlin;
use implementations::js;

mod language;
use language::{detect_language, Language};

mod git;
use git::{git, read_project_name};

mod github;
use github::read_remote_tests;

use colored::Colorize;
use yaml_rust::YamlLoader;

#[tokio::main]
async fn main() {
  // Show a warning message, just in case.
  println!("{}\n", "Welcome, please note that this tool is only intended to be used within the LiterateInk organization, since it expects a specific repository structure and provides no way to configure any feature.".yellow());

  //
  // Detect the language of the current implementation.
  // Also read the project name for later.
  // 

  let project_name = read_project_name();
  let language = detect_language();
  println!("Automatically detected {language} implementation for {project_name}.");

  //
  // Initialize the testing project in the directory.
  //

  match language {
    Language::Kotlin => kotlin::init_project(),
    Language::JsTs => js::init_project(),
    _ => panic!("unsupported language"),
  }

  //
  // Retrieve the tests from remote repository
  // and execute them in the testing project.
  //

  let tests = read_remote_tests(project_name).await.expect("Could not read tests from the remote repository.");
  for test_content in tests {
    let yml = YamlLoader::load_from_str(&test_content).unwrap();
    let yml = &yml[0];
  
    let boilerplate = yml["boilerplate"][language.to_branch_name()].as_str().unwrap();
    let tests = yml["tests"].as_vec().unwrap();
  
    for test in tests {
      let arguments = test["arguments"].as_vec().unwrap();
      let arguments: Vec<String> = arguments.iter().map(|arg| {
        let arg = arg.as_str().unwrap();
        let arg = format!("{:?}", arg);
        arg[1..arg.len() - 1].to_string()
      }).collect();

      let expected = test["expected"].as_str().unwrap();
      let result = match language {
        Language::Kotlin => {
          kotlin::use_boilerplate(boilerplate, arguments);
          kotlin::run_test()
        },
        Language::JsTs => {
          js::use_boilerplate(boilerplate, arguments);
          js::run_test()
        },
        _ => panic!("unsupported language"),
      };

      if result.trim() == expected {
        println!("PASSED: {}", test["name"].as_str().unwrap().green());
      } else {
        println!("FAILED: {}", test["name"].as_str().unwrap().red());
      }

      println!("Expected: {}", expected);
      println!("Received: {}", result);
    }
  }
  
  // Cleanup the testing project in the directory.
  match language {
    Language::Kotlin => kotlin::deinit_project(),
    Language::JsTs => js::deinit_project(),
    _ => panic!("unsupported language"),
  }
}
