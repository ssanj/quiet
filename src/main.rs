use std::{io::{self, BufRead}, convert::identity};
use clap::Parser;
use serde_json::Result as JsonResult;
use crate::models::{CompilerMessage, Reason, Cli};

mod models;

fn main() -> JsonResult<()>{
    let cli = Cli::parse();
    let errors_to_show = cli.errors as usize;
    let file_to_show_errors_for = cli.filter;

    let stdin = io::stdin();

    let matched: Vec<CompilerMessage> =
      stdin.lock()
        .lines()
        .into_iter()
        .filter_map(|line_result|{
          let line = line_result.unwrap();
          let reason: Reason = serde_json::from_str(&line).unwrap();

          //if  type of reason is compiler-message, then we want the full payload otherwise ignore?
          // we also want the build-finished
          if reason.reason == "compiler-message" {
            let compiler_message: CompilerMessage = serde_json::from_str(&line).unwrap();
            Some(compiler_message)
          } else {
            None
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

     filtered_match
      .into_iter()
      .take(errors_to_show)
      .for_each(|compiler_message|{
      println!("*** {} >>> {}", compiler_message.target.src_path, compiler_message.message.rendered);
      // println!("@@@@ {:#?}", compiler_message);

    });
    Ok(())
}
