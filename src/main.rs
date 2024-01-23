use std::{io::{self, BufRead}, collections::HashMap};
use clap::Parser;
use serde_json::Result as JsonResult;

use cli::Cli;
use compiler_message::CompilerMessage;
use compiler_message_decoding_status::CompilerMessageDecodingStatus;
use rendered::Rendered;

mod reason;
mod cli;
mod compiler_message;
mod compiler_message_decoding_status;
mod rendered;
mod process;


fn main() -> JsonResult<()>{
  let cli = Cli::parse();
  let items_to_show = cli.items as usize;
  let file_to_show_errors_for = cli.file_filter;
  let show_warnings = cli.show_warnings;

  process::stdout::print_start_banner();

  let matched: Vec<CompilerMessage> = get_matches();
  let filtered_match: Vec<CompilerMessage> = process::filter::by_filename(file_to_show_errors_for, matched);
  let level_info = process::level_status::get_messages_by_level(filtered_match);

  let constrained_matches: Vec<CompilerMessage> =
    process::limit::by_number(level_info.level_types, items_to_show, show_warnings);

  process::stdout::print_compiler_output(constrained_matches, level_info.status);

  Ok(())
}

fn get_matches() -> Vec<CompilerMessage> {
  let mut test_results_buffer: HashMap<&str, u32> = HashMap::new();

  process::get_compiler_messages()
  .into_iter()
  .filter_map(|r| {
    match r {
      Ok(CompilerMessageDecodingStatus::DecodedCompilerMessage(cm)) => Some(cm),
      Ok(CompilerMessageDecodingStatus::StdOutLine(line)) => {
        process::stdout::print_stdout_line(line.as_str(), &mut test_results_buffer);
        None
      },
      Ok(CompilerMessageDecodingStatus::NoCompilerMessage) => None,
      Err(e) => {
        println!("{}", e.to_string());
        None
      },
    }
  })
  .collect()
}

