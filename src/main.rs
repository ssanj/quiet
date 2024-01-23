use clap::Parser;
use serde_json::Result as JsonResult;

use cli::Cli;
use compiler_message::CompilerMessage;
use process::compiler_messages::{ItemTypes, get_matches};
use process::stdout::{print_start_banner, print_compiler_output, print_errors, print_stdout_lines};
use process::level_status::{by_level, LevelInfo};
use process::limit::by_number;
use process::filter::by_filename;
use process::all_messages::AllMessages;
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

  let all_messages = get_all_messages();
  let compiler_messages: Vec<CompilerMessage> = all_messages.compiler_messages;
  let filtered_by_filename: Vec<CompilerMessage> = by_filename(file_to_show_errors_for, compiler_messages);
  let level_info: LevelInfo = by_level(filtered_by_filename);
  let limited_by_item_size: Vec<CompilerMessage> =
    by_number(level_info.level_types, items_to_show, show_warnings);

  print_stdout_lines(all_messages.stdout_lines);
  print_errors(all_messages.errors);
  print_compiler_output(limited_by_item_size, level_info.status);

  Ok(())
}


fn get_all_messages() -> AllMessages {
  get_matches()
    .into_iter()
    .fold(AllMessages::new(), |mut acc: AllMessages, it| {
      match it {
        ItemTypes::CompilerMessageType(cm) => { acc.add_compiler_message(cm); acc },
        ItemTypes::StdoutLineType(line)    => { acc.add_stdout_line(line); acc },
        ItemTypes::ErrorType(error)        => { acc.add_error(error); acc },
      }
    })
}

