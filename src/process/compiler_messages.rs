use std::io::{stdin, BufRead};
use serde_json;
use crate::compiler_message::CompilerMessage;
use crate::reason::Reason;
use crate::compiler_message_decoding_status::CompilerMessageDecodingStatus;
use std::format as s;
use ansi_term::Color::Red;


#[allow(clippy::enum_variant_names)]
pub enum ItemTypes {
  CompilerMessageType(CompilerMessage),
  StdoutLineType(String),
  ErrorType(String)
}


pub fn get_matches() -> Vec<ItemTypes> {
  get_compiler_messages()
  .into_iter()
  .filter_map(|r| {
    match r {
      Ok(CompilerMessageDecodingStatus::DecodedCompilerMessage(cm)) => Some(ItemTypes::CompilerMessageType(cm)),
      Ok(CompilerMessageDecodingStatus::StdOutLine(line)) => {
        Some(ItemTypes::StdoutLineType(line))
      },
      Ok(CompilerMessageDecodingStatus::Ignore) => None,
      Err(e) => {
        Some(ItemTypes::ErrorType(e.to_string()))
      },
    }
  })
  .collect()
}


fn get_compiler_messages() -> Vec<Result<CompilerMessageDecodingStatus, String>> {
  stdin()
  .lock()
  .lines()
  .map(|line_result|{
    let line = line_result.unwrap();
    // if it's not a JSON payload
    if !&line.starts_with('{') {
      Ok(CompilerMessageDecodingStatus::StdOutLine(line))
    } else {
      let process_result: Result<CompilerMessageDecodingStatus, String> =
        process_compiler_message(line.as_str())
          .map(|maybe_cm| {
            maybe_cm.map_or_else(|| CompilerMessageDecodingStatus::Ignore, CompilerMessageDecodingStatus::DecodedCompilerMessage)
          });

      process_result
    }
  })
  .collect()
}


fn process_compiler_message(line: &str) -> Result<Option<CompilerMessage>, String> {
  let reason =
    decode_reason(line)
      .map_err(|e| {
          s!("******************* Failed to decode Reason from this line: {}\ncause: {}", Red.paint(line), e)
        })?;

  if reason.reason == "compiler-message" {
    decode_compiler_message(line)
      .map(Some)
      .map_err(|e| {
        s!("******************* Failed to decode CompilerMessage from this line: {}\ncause: {}", Red.paint(line), e)
      })
  } else {
    Ok(None)
  }
}


fn decode_reason(line: &str) -> serde_json::Result<Reason> {
  serde_json::from_str(line)
}


fn decode_compiler_message(line: &str) -> serde_json::Result<CompilerMessage> {
  serde_json::from_str(line)
}
