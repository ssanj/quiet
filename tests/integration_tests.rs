use assert_cmd::Command;
use predicates::prelude::predicate;
use std::{println as p, format as s};
use std::path::Path;


enum AssertionType<'a> {
  Contains(&'a str),
  DoesNotContain(&'a str),
}


#[test]
fn no_errors_1() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
    ];
  run_quiet("no-errors-1.txt", &stdout_lines)
}


#[test]
fn no_errors_2() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
    ];
  run_quiet("no-errors-2.txt", &stdout_lines)
}


#[test]
fn no_errors_3() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
    ];
  run_quiet("no-errors-3.txt", &stdout_lines)
}


#[test]
fn no_errors_4() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
    ];
  run_quiet("no-errors-4.txt", &stdout_lines)
}


#[test]
fn errors_1() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::DoesNotContain("*** No compilation errors"),
      AssertionType::Contains("*** /Volumes/Work/projects/code/rust/toy/purs/src/main.rs >>> error[E0412]: cannot find type `PullRequest` in this scope"),
      AssertionType::Contains("--> src/model.rs:68:23"),
      AssertionType::DoesNotContain("--> src/github.rs:15:69"),  // second error
      AssertionType::DoesNotContain("--> src/github.rs:88:23"),  // third error
      AssertionType::DoesNotContain("--> src/github.rs:114:36"), // fourth error
      AssertionType::DoesNotContain("--> src/main.rs:56:32"),    // fifth error
    ];
  run_quiet("errors-1.txt", &stdout_lines)
}


#[test]
fn no_errors_tests() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
      AssertionType::Contains("running 49 tests"),
      AssertionType::Contains("49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s"),
      AssertionType::Contains("running 12 tests"),
      AssertionType::Contains("12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s"),
      AssertionType::Contains("running 8 tests"),
      AssertionType::Contains("8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s"),
      AssertionType::Contains("running 0 tests"),
      AssertionType::Contains("0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s"),
    ];
  run_quiet("no-errors-tests.txt", &stdout_lines)
}


#[test]
fn errors_tests() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilation errors"),
      AssertionType::Contains("running 49 tests"),
      AssertionType::Contains("49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s"),
      AssertionType::Contains("running 12 tests"),
      AssertionType::Contains("12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s"),
      AssertionType::Contains("running 8 tests"),
      AssertionType::Contains("7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s"),
      AssertionType::Contains("runs_a_simple_template_with_shell_hook"),
      AssertionType::Contains("FAILED"),
    ];
  run_quiet("errors-tests.txt", &stdout_lines)
}


#[test]
fn compilation_errors_tests() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::DoesNotContain("*** No compilations errors"),
      AssertionType::Contains("*** /Volumes/Work/code/rust/toy/zat/tests/errors_integration_tests.rs >>>"),
      AssertionType::Contains("tests/errors_integration_tests.rs:171:67"),
      AssertionType::Contains("expected `;`, found keyword `let`"),
    ];
  run_quiet("compilation-errors-tests.txt", &stdout_lines)
}


fn run_quiet<P: AsRef<Path>>(cargo_output_file: P, stdout_assertions: &[AssertionType]) {
  let mut cmd = Command::cargo_bin("quiet").unwrap();

  let stdout_contains = |expected: &str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");

      let result = output.contains(owned_expected.as_str());
      // Only print this if the result is not what we expect, otherwise this will get printed for any test failure.
      if !result {
        // This only gets printed if the test fails.
        p!(">>> stdout: {}\n>>> did not contain: '{}'\n", output, owned_expected.as_str());
      }
      result
    })
  };


  let stdout_does_not_contain = |expected: &str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");

      let result = !output.contains(owned_expected.as_str());
      if !result {
        // This only gets printed if the test fails.
        p!(">>> stdout: {}\n>>> contained: '{}'\n", output, owned_expected.as_str());
      }

      result
    })
  };

  let example_file = get_example_file(&cargo_output_file);
  let cargo_output =
    std::fs::read(example_file.as_str())
    .expect(&s!("Could not read file {}", &example_file));

  let input = std::str::from_utf8(&cargo_output).expect("Could not decode output");

  cmd
    .arg("--items")
    .arg("1")
    .write_stdin(input);

  let mut asserts =
    cmd
      .assert()
      .success();

  let lines_should_exist =
    stdout_assertions
      .iter()
      .filter_map(|at| {
        match at {
          AssertionType::Contains(m) => Some(m),
          AssertionType::DoesNotContain(_) => None,
        }
      });

  let lines_should_not_exist =
    stdout_assertions
      .iter()
      .filter_map(|at| {
        match at {
          AssertionType::Contains(_) => None,
          AssertionType::DoesNotContain(m) => Some(m),
        }
      });

  for line in lines_should_exist{
    asserts =
      asserts
        .stdout(stdout_contains(line));
  }

  for line in lines_should_not_exist{
    asserts =
      asserts
        .stdout(stdout_does_not_contain(line));
  }
}


fn get_example_file<P: AsRef<Path>>(file: P) -> String {
  let current_directory = std::env::current_dir().expect("Could not get current directory");
  println!("current directory {}", current_directory.to_string_lossy());

  let example_file = current_directory.join(Path::new("tests/example").join(file));
  println!("example file {}", example_file.to_string_lossy());

  example_file.to_string_lossy().to_string()
}
