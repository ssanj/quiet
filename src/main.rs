use std::{io::{self, BufRead}, collections::HashMap};
use clap::Parser;
use serde_json::Result as JsonResult;
use ansi_term::Colour::{Green, Red, Blue, RGB, Yellow};
use std::time::SystemTime;

use cli::Cli;
use compiler_message::CompilerMessage;
use reason::Reason;
use std::format as s;
use itertools::Itertools;
use rendered::Rendered;

mod reason;
mod cli;
mod compiler_message;
mod rendered;


fn main() -> JsonResult<()>{
  let cli = Cli::parse();
  let items_to_show = cli.items as usize;
  let file_to_show_errors_for = cli.file_filter;
  let show_warnings = cli.show_warnings;

  print_start_banner();

  let matched: Vec<CompilerMessage> =
    get_compiler_messages()
    .into_iter()
    .filter_map(|r| {
      match r {
        Ok(CompilerMessageDecodingStatus::DecodedCompilerMessage(cm)) => Some(cm),
        Ok(CompilerMessageDecodingStatus::NoCompilerMessage) => None,
        Err(e) => {
          println!("{}", e.to_string());
          None
        },
      }
    })
    .collect();

  let filtered_match: Vec<CompilerMessage> = filter_by_filename(file_to_show_errors_for, matched);
  let filtered_by_level: Vec<LevelType> = filter_by_level(filtered_match);
  let level_status: LevelStatus = get_level_status(&filtered_by_level);

  let constrained_matches: Vec<CompilerMessage> =
    get_constrained_by_number(filtered_by_level, items_to_show, show_warnings);

  print_compiler_output(constrained_matches, level_status);

  Ok(())
}


fn get_level_status(filtered_by_level: &[LevelType]) -> LevelStatus {
  let init =
    LevelStatus {
      errors: false,
      warnings: false,
    };

  let result =
    filtered_by_level
      .iter()
      .fold(init, |acc, v| {
        match v {
          LevelType::ErrorLevel(_) => {
            if !acc.errors {
              acc.copy_errors(true)
            } else {
              acc
            }
          },
          LevelType::WarningLevel(_) => {
            if !acc.warnings {
              acc.copy_warnings(true)
            } else {
              acc
            }
          },
        }
      });

  result
}

#[derive(Debug, Clone, PartialEq)]
enum LevelType {
  ErrorLevel(CompilerMessage),
  WarningLevel(CompilerMessage),
}

struct LevelStatus {
  errors: bool,
  warnings: bool
}

impl LevelStatus {
  fn copy_errors(self, new_errors: bool) -> Self {
    Self {
      errors: new_errors,
      warnings: self.warnings
    }
  }

  fn copy_warnings(self, new_warnings: bool) -> Self {
    Self {
      errors: self.errors,
      warnings: new_warnings
    }
  }
}

enum OutputType<'a> {
  Error(&'a str),
  Warning(&'a str),
  Success(&'a str),
}

fn print_compiler_output(constrained_matches: Vec<CompilerMessage>, level_status: LevelStatus) {
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


fn filter_by_level(filtered_match: Vec<CompilerMessage>) -> Vec<LevelType> {
  filtered_match
    .into_iter()
    .filter_map(|i| {
      let level = &i.message.level;
      match level.as_str() {
        "error"   => Some(LevelType::ErrorLevel(i)),
        "warning" => Some(LevelType::WarningLevel(i)),
        _         => None
      }
    })
    .collect()
}


fn get_constrained_by_number(mut filtered_by_level: Vec<LevelType>, items_to_show: usize, show_warnings: bool) -> Vec<CompilerMessage> {
  if !show_warnings {
    // Errors only
    filtered_by_level
      .into_iter()
      .filter_map(|lt| {
        match lt {
          LevelType::ErrorLevel(cm) => Some(cm),
          _                         => None,
        }
      })
      .take(items_to_show)
      .collect()
  } else {
      // Both errors and warnings
      // Sort with errors first, then warnings
      filtered_by_level
      .sort_by_key(|lt|{
        match lt {
          LevelType::ErrorLevel(_)   => 0,
          LevelType::WarningLevel(_) => 1
        }
      });

      // The warnings returned by the Cargo JSON have duplicate elements.
      // We convert them to Rendered to allow us to remove duplicated with the same rendered output.
      filtered_by_level
        .into_iter()
        .filter_map(|lt|{
          match lt {
            LevelType::ErrorLevel(cm)   => Some(Rendered::new(cm)),
            LevelType::WarningLevel(cm) => {
              if cm.message.message.contains("warning emitted") { // Also remove messages that say "warning emitted"
                None
              } else {
                Some(Rendered::new(cm))
              }
            },
          }
        })
        .unique() // Removes duplicates
        .map(|r| r.items)
        .take(items_to_show)
        .collect()
  }
}


fn filter_by_filename(file_to_show_errors_for: Option<String>, matched: Vec<CompilerMessage>) -> Vec<CompilerMessage> {
  match file_to_show_errors_for {
    Some(file_name_filter) => {
      matched
        .into_iter()
        .filter(|compiler_message|{
          let filter_matches =
            compiler_message
              .message
              .spans
              .iter()
              .filter(|span|{
                span
                  .file_name
                  .ends_with(&file_name_filter)
                });

           let has_matches = !filter_matches.collect::<Vec<_>>().is_empty();
           has_matches
        })
        .collect::<Vec<_>>()
    },
    None => matched
  }
}

enum ReasonDecodingStatus<'a> {
  MatchesCompilerMessage(&'a str),
  DoesNotMatchCompilerMessage
}

enum CompilerMessageDecodingStatus {
  DecodedCompilerMessage(CompilerMessage),
  NoCompilerMessage
}


fn get_compiler_messages() -> Vec<Result<CompilerMessageDecodingStatus, String>> {
  let mut test_results_buffer: HashMap<&str, u32> = HashMap::new();

  io::stdin()
  .lock()
  .lines()
  .into_iter()
  .map(|line_result|{
    let line = line_result.unwrap();
    // if it's not a JSON payload
    if !&line.starts_with("{") {
      // Maybe use an ADT and tag this as StdoutMessage vs JsonMessage
      passthrough_stdout_line(line.as_str(), &mut test_results_buffer);
      Ok(CompilerMessageDecodingStatus::NoCompilerMessage)
    } else {
      let reason_decoding: Result<(&str, Reason), String> =
        decode_reason(line.as_str())
          .map(|r| (line.as_str(), r))
          .map_err(|e| {
              s!("******************* Failed to decode Reason from this line: {}\ncause: {}", Red.paint(line.as_str()), e.to_string())
            });


      let decoding_status: Result<ReasonDecodingStatus<'_>, String> =
        reason_decoding
          .map(|(l, reason)| {
            if reason.reason == "compiler-message" {
              ReasonDecodingStatus::MatchesCompilerMessage(l)
            } else {
              ReasonDecodingStatus::DoesNotMatchCompilerMessage
            }
          });


      let result: Result<CompilerMessageDecodingStatus, String> =
        decoding_status
          .and_then(|ds| {
            match ds {
              ReasonDecodingStatus::MatchesCompilerMessage(l) => {
                  decode_compiler_message(l)
                  .map(|cm| CompilerMessageDecodingStatus::DecodedCompilerMessage(cm))
                  .map_err(|e| {
                    s!("******************* Failed to decode CompilerMessage from this line: {}\ncause: {}", Red.paint(l), e.to_string())
                  })
              },
              ReasonDecodingStatus::DoesNotMatchCompilerMessage => Ok(CompilerMessageDecodingStatus::NoCompilerMessage),
            }
          });

      result
    } //else
  })
  .collect()
}


fn passthrough_stdout_line(line: &str, test_results_buffer: &mut HashMap<&str, u32>) {
  if let Some(new_line) = updated_stdout_line(&line, test_results_buffer) {
    println!("{}", new_line)
  }
}


fn decode_reason(line: &str) -> serde_json::Result<Reason> {
  serde_json::from_str(&line)
}


fn decode_compiler_message(line: &str) -> serde_json::Result<CompilerMessage> {
  serde_json::from_str(&line)
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


/// Help identify the current execution of quiet by using a unique number for each execution.
/// This can be useful for when you are fixing a lot of errors one by one, and have a lot of
/// compilation errors on the screen.
fn print_start_banner() {
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
