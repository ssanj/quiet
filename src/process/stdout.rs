use std::collections::HashMap;
use std::format as s;
use ansi_term::Color::{Red, Green, Yellow, Blue, RGB};
use crate::CompilerMessage;
use super::level_status::LevelStatus;
use std::time::SystemTime;


pub fn print_stdout_line(line: &str, test_results_buffer: &mut HashMap<&str, u32>) {
  if let Some(new_line) = updated_stdout_line(&line, test_results_buffer) {
    println!("{}", new_line)
  }
}


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


enum OutputType<'a> {
  Error(&'a str),
  Warning(&'a str),
  Success(&'a str),
}


// TODO: Refactor this spaghetti code
fn updated_stdout_line(line: &str, test_results_buffer: &mut HashMap<&str, u32>) -> Option<String> {
  if line.is_empty() {
    None
  } else if line == "failures:" {
    let dots = print_success_dots(test_results_buffer.get("success"));
    Some(print_failures_line(line, dots.as_deref()))
  } else if line.starts_with("test result: FAILED.") {
    // Clear the test success
    test_results_buffer.clear();
    Some(print_test_failure(line))
  } else if line.starts_with("test result: ok.") {
    // Print out the collected tests
    let dots = print_success_dots(test_results_buffer.get("success"));
    let output = print_test_success(line, dots.as_deref());
    test_results_buffer.clear();
    Some(output)
  } else if line.split_inclusive(".").count() == line.len()  {
    Some(print_test_run_dots(line))
  } else if line.trim().starts_with("Finished ") ||
            line.trim().starts_with("Compiling ") ||
            line.trim().starts_with("error: ") ||
            line.trim().starts_with("warning: ") {
    None
  } else if line.trim().starts_with("Running ") {
    Some(print_test_name(line))
  } else if line.ends_with("... ok") {
    // TODO: Move to a function
    let existing_success_count = test_results_buffer.get("success");
    if let Some(success_count) = existing_success_count {
      test_results_buffer.insert("success", success_count + 1);
    } else {
      test_results_buffer.insert("success", 1);
    }
    None
  } else if line.ends_with("... FAILED") {
    Some(print_failed_test_name(line))
  } else {
    Some(default_stdout_line(line))
  }
}


fn print_failed_test_name(line: &str) -> String {
  s!("{}", Red.paint(line))
}


fn print_test_name(line: &str) -> String {
  s!("\n{}", Yellow.paint(line.trim().strip_prefix("Running ").unwrap_or(line)))
}


fn print_test_run_dots(line: &str) -> String {
  s!("{}", Green.paint(line))
}

fn print_failures_line(line: &str, maybe_dots: Option<&str>) -> String {
  match maybe_dots {
    Some(dots) => s!("{}\n{} {}", dots, RGB(133, 138, 118).paint("stdout:"), Red.paint(line)),
    None => s!("{} {}", RGB(133, 138, 118).paint("stdout:"), Red.paint(line)),
  }
}


fn print_test_failure(line: &str) -> String {
  let failure = s!("test result: {}.", Red.paint("FAILED"));
  let message = s!("{}{}", failure, line.strip_prefix("test result: FAILED.").unwrap_or_else(|| ""));
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message)
}


fn print_success_dots(successes: Option<&u32>) -> Option<String> {
  successes
    .map(|dots_count|{
      let dots_str = (0 .. *dots_count).into_iter().map(|_| ".").collect::<String>();
      s!("{}", Green.paint(dots_str))
    })
}


fn print_test_success(line: &str, maybe_dots: Option<&str>) -> String {
  let test_result = s!("test result: {}.", Green.paint("ok"));
  let message = s!("{}{}", test_result, line.strip_prefix("test result: ok.").unwrap_or_else(|| ""));

  let formatted_test_result = s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message);
  match maybe_dots {
    Some(dots) => s!("{}\n{}", &dots, &formatted_test_result),
    None => s!("{}", &formatted_test_result),
}

}


fn default_stdout_line(line: &str) -> String {
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), line)
}
