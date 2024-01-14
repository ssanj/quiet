use std::{io::{self, BufRead}};
use clap::Parser;
use serde_json::Result as JsonResult;
use ansi_term::Colour::{Green, Red, Blue, RGB};
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
  let errors_to_show = cli.errors as usize;
  let file_to_show_errors_for = cli.file_filter;
  let show_warnings = cli.show_warnings;
  print_start_banner();

  let matched: Vec<CompilerMessage> = get_compiler_messages();

  let filtered_match  =
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
    };

   let errors_only =
    filtered_match
      .into_iter()
      .filter(|i| {
        let level = &i.message.level;
        let default_level = "error";
        if show_warnings {
          level == default_level || level == "warning"
        } else {
          level == default_level
        }
      })
      .take(errors_to_show);

  // We could have mapped to get the output Strings and then counted + printed. But there's no good reason to.
  // Using a mut here is simpler and Rust gives us the guarantees we need.
  let mut printed_items = 0;
  errors_only
      .for_each(|compiler_message|{
        println!("*** {} >>> {}", compiler_message.target.src_path, compiler_message.message.rendered);
        printed_items += 1;
      });

  if printed_items == 0 {
    let prefix = "*** No compilations errors";
    let message =
      if show_warnings {
        s!("{} or warnings", prefix)
      } else {
        prefix.to_owned()
      };

    println!("{}", Green.paint(message))
  }

  Ok(())
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
