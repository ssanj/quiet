use std::collections::HashMap;
use std::format as s;
use ansi_term::Color::{Red, Green, Yellow, Blue, RGB};
use crate::CompilerMessage;
use super::level_status::LevelStatus;
use std::time::SystemTime;


/// Help identify the current execution of quiet by using a unique number for each execution.
/// This can be useful for when you are fixing a lot of errors one by one, and have a lot of
/// compilation errors on the screen.
pub fn print_start_banner() {
  println!();
  let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_millis()).expect("EPOCH is before current time. What?!?");
  let time_str = s!("{}", time);
  let id: String =
    time_str
      .chars()
      .rev()
      .take(7)
      .collect();

  let id_string = s!("---------- quiet [{}]----------", id);
  println!("{}", Blue.paint(id_string));
}


pub fn print_compiler_output(constrained_matches: Vec<CompilerMessage>, level_status: LevelStatus) {
  println!();
  constrained_matches
    .into_iter()
    .for_each(|compiler_message|{
      println!("*** {} >>> {}", compiler_message.target.src_path, compiler_message.message.rendered)
    });

  let output_type =
    match (level_status.errors, level_status.warnings) {
      (true, true)  => OutputType::Error("!!! There are compilation errors and warnings !!!"),
      (true, false) => OutputType::Error("!!! There are compilation errors !!!"),
      (false, true) => OutputType::Warning("*** No compilation errors (but there are warnings) ***"),
      (false, false) => OutputType::Success("*** No compilation errors (or warnings) ***"),
    };

  match output_type {
    OutputType::Error(m)   => println!("\n{}", Red.paint(m)),
    OutputType::Warning(m) => println!("\n{}", Yellow.paint(m)),
    OutputType::Success(m) => println!("\n{}", Green.paint(m)),
  }
}

pub fn print_errors(errors: Vec<String>) {
    errors
      .into_iter()
      .for_each(|e| {
        println!("{}", e)
      })
}


pub fn print_stdout_lines(stdout_lines: Vec<String>) {
  let line_types: Vec<LineType> = get_line_types(stdout_lines);
  let stdout_lines: Vec<String> = get_stdout_lines(line_types);

  stdout_lines
    .into_iter()
    .for_each(|line| println!("{}", line))
}


fn get_stdout_lines(line_types: Vec<LineType>) -> Vec<String> {
  let mut test_results_buffer: HashMap<&str, u32> = HashMap::new();

  line_types
    .into_iter()
    .filter_map(|line_type| {
      match line_type {
        LineType::Empty => None,
        LineType::Failures(line) => {
          let dots = success_dots_string(test_results_buffer.get("success"));
          Some(failure_line_string(line.as_str(), dots.as_deref()))
        },
        LineType::TestResultFailed(line) => {
          // Clear the test success
          test_results_buffer.clear();
          Some(test_failure_string(line.as_str()))
        },
        LineType::TestResultOk(line) => {
          // Print out the collected tests
          let dots = success_dots_string(test_results_buffer.get("success"));
          let output = test_success_string(line.as_str(), dots.as_deref());
          test_results_buffer.clear();
          Some(output)
        },
        LineType::TestDots(line) => Some(test_run_dots_string(line.as_str())),
        LineType::Finished(_) => None,
        LineType::Compiling(_) => None,
        LineType::Error(_) => None,
        LineType::Warning(_) => None,
        LineType::Running(line) => Some(test_name_string(line.as_str())),
        LineType::SingleTestOk(_) => {
          // TODO: Move to a function
          let existing_success_count = test_results_buffer.get("success");
          if let Some(success_count) = existing_success_count {
            test_results_buffer.insert("success", success_count + 1);
          } else {
            test_results_buffer.insert("success", 1);
          }
          None
        },
        LineType::SingleTestFailed(line) => Some(failed_test_name_string(line.as_str())),
        LineType::Unprocessed(line) => Some(default_stdout_string(line.as_str())),
      }
    })
    .collect()
}


fn get_line_types(stdout_lines: Vec<String>) -> Vec<LineType> {
  stdout_lines
    .into_iter()
    .map(|line| {
      if line.is_empty() {
        LineType::Empty
      } else if line == "failures:" {
        LineType::Failures(line)
      } else if line.starts_with("test result: FAILED.") {
        LineType::TestResultFailed(line)
      } else if line.starts_with("test result: ok.") {
        LineType::TestResultOk(line)
      } else if line.split_inclusive(".").count() == line.len()  {
        LineType::TestDots(line)
      } else if line.trim().starts_with("Finished ") {
        LineType::Finished(line)
      } else if line.trim().starts_with("Compiling ") {
        LineType::Compiling(line)
      } else if line.trim().starts_with("error: ") {
        LineType::Error(line)
      } else if line.trim().starts_with("warning: ") {
        LineType::Warning(line)
      } else if line.trim().starts_with("Running ") {
        LineType::Running(line)
      } else if line.ends_with("... ok") {
        LineType::SingleTestOk(line)
      } else if line.ends_with("... FAILED") {
        LineType::SingleTestFailed(line)
      } else {
        LineType::Unprocessed(line)
      }
    })
    .collect()
}


enum OutputType<'a> {
  Error(&'a str),
  Warning(&'a str),
  Success(&'a str),
}

enum LineType {
  Empty,
  Failures(String),
  TestResultFailed(String),
  TestResultOk(String),
  TestDots(String),
  Finished(String),
  Compiling(String),
  Error(String),
  Warning(String),
  Running(String),
  SingleTestOk(String),
  SingleTestFailed(String),
  Unprocessed(String),
}


fn failed_test_name_string(line: &str) -> String {
  s!("{}", Red.paint(line))
}


fn test_name_string(line: &str) -> String {
  s!("\n{}", Yellow.paint(line.trim().strip_prefix("Running ").unwrap_or(line)))
}


fn test_run_dots_string(line: &str) -> String {
  s!("{}", Green.paint(line))
}

fn failure_line_string(line: &str, maybe_dots: Option<&str>) -> String {
  match maybe_dots {
    Some(dots) => s!("{}\n{} {}", dots, RGB(133, 138, 118).paint("stdout:"), Red.paint(line)),
    None => s!("{} {}", RGB(133, 138, 118).paint("stdout:"), Red.paint(line)),
  }
}


fn test_failure_string(line: &str) -> String {
  let failure = s!("test result: {}.", Red.paint("FAILED"));
  let message = s!("{}{}", failure, line.strip_prefix("test result: FAILED.").unwrap_or_else(|| ""));
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message)
}


fn success_dots_string(successes: Option<&u32>) -> Option<String> {
  successes
    .map(|dots_count|{
      let dots_str = (0 .. *dots_count).into_iter().map(|_| ".").collect::<String>();
      s!("{}", Green.paint(dots_str))
    })
}


fn test_success_string(line: &str, maybe_dots: Option<&str>) -> String {
  let test_result = s!("test result: {}.", Green.paint("ok"));
  let message = s!("{}{}", test_result, line.strip_prefix("test result: ok.").unwrap_or_else(|| ""));

  let formatted_test_result = s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message);
  match maybe_dots {
    Some(dots) => s!("{}\n{}", &dots, &formatted_test_result),
    None => s!("{}", &formatted_test_result),
  }
}


fn default_stdout_string(line: &str) -> String {
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), line)
}
