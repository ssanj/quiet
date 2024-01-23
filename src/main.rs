use std::{io::{self, BufRead}, collections::HashMap};
use clap::Parser;
use serde_json::Result as JsonResult;

use cli::Cli;
use compiler_message::CompilerMessage;
use process::compiler_messages::{ItemTypes, get_matches};
use process::stdout::{print_start_banner, print_stdout_line, print_compiler_output};
use process::level_status::get_messages_by_level;
use process::limit::by_number;
use process::filter::by_filename;
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

  print_start_banner();

  let compiler_messages: Vec<CompilerMessage> = get_compiler_messages();
  let filtered_by_filename: Vec<CompilerMessage> = by_filename(file_to_show_errors_for, compiler_messages);
  let level_info = get_messages_by_level(filtered_by_filename);
  let limited_by_item_size: Vec<CompilerMessage> =
    by_number(level_info.level_types, items_to_show, show_warnings);

  print_compiler_output(limited_by_item_size, level_info.status);

  Ok(())
}


fn get_compiler_messages() -> Vec<CompilerMessage> {
  let mut test_results_buffer: HashMap<&str, u32> = HashMap::new();

  get_matches()
    .into_iter()
    .filter_map(|it| {
      match it {
        ItemTypes::CompilerMessageType(cm) => Some(cm),
        ItemTypes::StdoutLineType(line) => {
          print_stdout_line(line.as_str(), &mut test_results_buffer);
          None
        },
        ItemTypes::ErrorType(error) => {
          println!("{}", error);
          None
        },
      }
    })
    .collect()
}

