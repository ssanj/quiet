use std::{io::{self, BufRead}};
use clap::Parser;
use serde_json::Result as JsonResult;
use ansi_term::Colour::{Green, Red, Blue, RGB, Yellow};
use std::time::SystemTime;

use cli::Cli;
use compiler_message::CompilerMessage;
use reason::Reason;
use std::format as s;

mod reason;
mod cli;
mod compiler_message;


fn main() -> JsonResult<()>{
  let cli = Cli::parse();
  let items_to_show = cli.items as usize;
  let file_to_show_errors_for = cli.file_filter;
  let show_warnings = cli.show_warnings;

  print_start_banner();

  let matched: Vec<CompilerMessage> = get_compiler_messages();
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
    OutputType::Error(m)   => println!("{}", Red.paint(m)),
    OutputType::Warning(m) => println!("{}", Yellow.paint(m)),
    OutputType::Success(m) => println!("{}", Green.paint(m)),
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

      filtered_by_level
        .into_iter()
        .map(|lt|{
          match lt {
            LevelType::ErrorLevel(cm)   => cm,
            LevelType::WarningLevel(cm) => cm,
          }
        })
        .take(items_to_show) // TODO: Accept a separate number of warnings to show?
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


fn get_compiler_messages() -> Vec<CompilerMessage> {
  io::stdin()
    .lock()
    .lines()
    .into_iter()
    .filter_map(|line_result|{
      let line = line_result.unwrap();
      // if it's not a JSON payload
      if !&line.starts_with("{") {
        // Maybe use an ADT and tag this as StdoutMessage vs JsonMessage
        passthrough_stdout_line(line.as_str());
        None
      } else {
        let reason: Reason = decode_reason(line.as_str());

        //if type of reason is compiler-message, then we want the full payload otherwise ignore.
        if reason.reason == "compiler-message" {
          let compiler_message = decode_compiler_message(line.as_str());
          Some(compiler_message)
        } else {
          None
        }
      }
    }).collect()
}


fn passthrough_stdout_line(line: &str) {
  let new_line = updated_stdout_line(&line);
  println!("{}", new_line);
}


fn decode_reason(line: &str) -> Reason {
  let line_with_decoding_error = s!("******************* Failed to decode Reason from this line: {}", Red.paint(line));

  serde_json::from_str(&line).expect(&line_with_decoding_error)
}


fn decode_compiler_message(line: &str) -> CompilerMessage {
  let line_with_error = s!("******************* Failed to decode CompilerMessage from this line: {}", Red.paint(line));
  // Dump out line if this result fails so we know where to look
  serde_json::from_str(&line).expect(&line_with_error)
}


fn updated_stdout_line(line: &str) -> String {
  if line == "failures:" {
    print_failures_line(line)
  } else if line.starts_with("test result: FAILED.") {
    print_test_failure(line)
  } else if line.starts_with("test result: ok.") {
    print_test_success(line)
  } else {
    default_stdout_line(line)
  }
}


fn print_failures_line(line: &str) -> String {
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), Red.paint(line))
}


fn print_test_failure(line: &str) -> String {
  let failure = s!("test result: {}.", Red.paint("FAILED"));
  let message = s!("{}{}", failure, line.strip_prefix("test result: FAILED.").unwrap_or_else(|| ""));
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message)
}


fn print_test_success(line: &str) -> String {
  let test_result = s!("test result: {}.", Green.paint("ok"));
  let message = s!("{}{}", test_result, line.strip_prefix("test result: ok.").unwrap_or_else(|| ""));
  s!("{} {}", RGB(133, 138, 118).paint("stdout:"), message)
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
