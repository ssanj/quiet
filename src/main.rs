use std::{io::{self, BufRead}, convert::identity};
use serde_json::Result as JsonResult;
use crate::models::{CompilerMessage, Reason};
mod models;

fn main() -> JsonResult<()>{
    let stdin = io::stdin();

    let errors_to_show = 1;
    let file_to_show_errors_for = Some("github.rs");

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

    matched
      .into_iter()
      .filter(|compiler_message|{
        let filter_result: bool =
          file_to_show_errors_for.map(|file_name_filter|{
            let filter_matches = compiler_message.message.spans.iter().filter(|span|{
              span.file_name.ends_with(file_name_filter)
            });

            !filter_matches.collect::<Vec<_>>().is_empty()
        }).map_or(false, identity);

        filter_result
      })
      .take(errors_to_show)
      .for_each(|compiler_message|{
      println!("*** {} >>> {}", compiler_message.target.src_path, compiler_message.message.rendered);
      // println!("@@@@ {:#?}", compiler_message);

    });
    Ok(())
}
