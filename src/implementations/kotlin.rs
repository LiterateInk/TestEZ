use std::{fs::{create_dir_all, remove_dir_all, File}, io::{Read, Write}};

const BUILD_GRADLE_APPEND: &str = r#"
plugins {
  id("org.jetbrains.kotlin.jvm")
  application
}

dependencies {
  implementation(project(":library"))
  implementation("ch.qos.logback:logback-classic:1.5.6")
  implementation("org.jetbrains.kotlinx:kotlinx-serialization-core:1.7.1")
  implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.1")
}

application {
  mainClass = "MainKt"
}

tasks {
  val fatJar = register<Jar>("fatJar") {
    dependsOn.addAll(listOf("compileJava", "compileKotlin", "processResources"))
    archiveClassifier.set("standalone")
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
    manifest { attributes(mapOf("Main-Class" to application.mainClass)) }
    val sourcesMain = sourceSets.main.get()
    val contents = configurations.runtimeClasspath.get()
      .map { if (it.isDirectory) it else zipTree(it) } +
      sourcesMain.output
    from(contents)
  }
  build {
    dependsOn(fatJar)
  }
}
"#;

fn read_settings_gradle_kts () -> String {
  let mut file = File::open("settings.gradle.kts").unwrap();
  let mut buffer = String::new();
  file.read_to_string(&mut buffer).unwrap();
  buffer
}

fn write_settings_gradle_kts (content: &str) {
  let mut file = File::create("settings.gradle.kts").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

pub fn init_project () {
  let mut settings_content = read_settings_gradle_kts();
  settings_content.push_str("include(\":testez\")");
  write_settings_gradle_kts(&settings_content);

  create_dir_all("testez/src/main/kotlin").unwrap();
  let mut file = File::create("testez/build.gradle.kts").unwrap();
  file.write_all(BUILD_GRADLE_APPEND.as_bytes()).unwrap();
}

pub fn deinit_project () {
  let settings_content = read_settings_gradle_kts();
  let settings_content = settings_content.replace("include(\":testez\")", "");
  write_settings_gradle_kts(&settings_content);
  remove_dir_all("testez").unwrap();
}

pub fn use_boilerplate (boilerplate: &str, arguments: Vec<String>) {
  let mut content = boilerplate.to_string();

  for (index, argument) in arguments.iter().enumerate() {
    content = content.replace(&format!("[#{}#]", index), argument);
  }

  let mut file = File::create("testez/src/main/kotlin/Main.kt").unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

pub fn run_test () -> String {
  let output = std::process::Command::new("./gradlew")
    .args([":testez:build"])
    .output()
    .expect("failed to run gradle command, make sure gradle is installed on your machine");

    if !output.stderr.is_empty() {
      let error = String::from_utf8_lossy(&output.stderr);
      panic!("Happened during 'gradlew build':\n{}", error);
    }

  let output = std::process::Command::new("java")
    .args(["-jar", "./testez/build/libs/testez-standalone.jar"])
    .output()
    .expect("failed to run java command, make sure java is installed on your machine");

  if !output.stderr.is_empty() {
    let error = String::from_utf8_lossy(&output.stderr);
    panic!("Happened while running the .jar (in runtime):\n{}", error);
  }

  let output = String::from_utf8_lossy(&output.stdout);
  output.to_string()
}