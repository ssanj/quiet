use assert_cmd::Command;
use assert_cmd::{cargo::CommandCargoExt, assert::OutputAssertExt, output::OutputOkExt};
use predicates::prelude::predicate;
use std::{println as p, format as s};
use std::path::Path;

enum AssertionType<'a> {
  Contains(&'a str),
  DoesNotContain(&'a str),
}

#[test]
fn without_any_errors() {
  let stdout_lines =
    [
      AssertionType::Contains("quiet"),
      AssertionType::Contains("*** No compilations errors"),
    ];
  run_quiet("no-errors.txt", &stdout_lines)
}

fn run_quiet<P: AsRef<Path>>(cargo_output_file: P, stdout_assertions: &[AssertionType]) {
  let mut cmd = Command::cargo_bin("quiet").unwrap();

  let stdout_contains = |expected: &str| {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");

      let result = output.contains(owned_expected.as_str());
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

      //result
      false
    })
  };

  let example_file = get_example_file(&cargo_output_file);
  let cargo_output =
    std::fs::read(example_file.as_str())
    .expect(&s!("Could not read file {}", &example_file));

  let input = std::str::from_utf8(&cargo_output).expect("Could not decode output");

  cmd
    .arg("--errors")
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
        .stderr(stdout_does_not_contain(line));
  }
}

fn get_example_file<P: AsRef<Path>>(file: P) -> String {
  let current_directory = std::env::current_dir().expect("Could not get current directory");
  println!("current directory {}", current_directory.to_string_lossy());

  let example_file = current_directory.join(Path::new("tests/example").join(file));
  println!("example file {}", example_file.to_string_lossy());

  example_file.to_string_lossy().to_string()
}
