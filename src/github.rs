use serde_json::Value;

async fn read_remote_tests_tree (project_name: &str) -> Result<Vec<String>, reqwest::Error> {
  let url = format!("https://api.github.com/repos/LiterateInk/{project_name}/contents/?ref=tests");

  let request = reqwest::Client::new()
    .get(&url)
    .header("User-Agent", "LiterateInk")
    .header("Accept", "application/vnd.github+json");


  let body = request.send()
    .await?
    .text()
    .await?;

  let json: Value = serde_json::from_str(&body).unwrap();
  let json = json.as_array().unwrap();

  let files: Vec<String> = json.iter()
    .map(|item| item["name"].as_str().unwrap().to_string())
    .collect();

  Ok(files)
}

pub async fn read_remote_tests (project_name: String) -> Result<Vec<String>, reqwest::Error> {
  let files = read_remote_tests_tree(&project_name).await.unwrap();
  let mut tests: Vec<String> = vec![];

  for file_path in files {
    let url = format!("https://raw.githubusercontent.com/LiterateInk/{project_name}/refs/heads/tests/{file_path}");
    
    let body = reqwest::get(url)
      .await?
      .text()
      .await?;

    tests.push(body);
  }

  Ok(tests)
}
