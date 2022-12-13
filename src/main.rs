use std::{io::{self, BufRead}, convert::identity};
use clap::Parser;
use serde_json::Result as JsonResult;
use crate::models::{CompilerMessage, Reason, Cli};
use ansi_term::Colour::{Green, Red, RGB};

mod models;

fn main() -> JsonResult<()>{
  let cli = Cli::parse();
  let errors_to_show = cli.errors as usize;
  let file_to_show_errors_for = cli.file_filter;
  let show_warnings = cli.show_warnings;

  let stdin = io::stdin();

  let matched: Vec<CompilerMessage> =
    stdin.lock()
      .lines()
      .into_iter()
      .filter_map(|line_result|{
        let line = line_result.unwrap();
        if !&line.starts_with("{") {
          println!("{} {}", RGB(133, 138, 118).paint("stdout:"), &line); // Not json, just output it and proceed to the next line
          None
        } else {
          let line_with_decoding_error = format!("******************* Failed to decode Reason from this line: {}", Red.paint(&line));
          let reason: Reason = serde_json::from_str(&line).expect(&line_with_decoding_error);

          //if  type of reason is compiler-message, then we want the full payload otherwise ignore?
          // we also want the build-finished
          if reason.reason == "compiler-message" {
            let line_with_error = format!("******************* Failed to decode CompilerMessage from this line: {}", Red.paint(&line));
            // Dump out line if this result fails so we know where to look
            let compiler_message: CompilerMessage = serde_json::from_str(&line).expect(&line_with_error);
            Some(compiler_message)
          } else {
            None
          }
        }
      }).collect();

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
        format!("{} or warnings", prefix)
      } else {
        prefix.to_owned()
      };

    println!("{}", Green.paint(message))
  }

  Ok(())
}
