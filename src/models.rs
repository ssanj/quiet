use serde::{Serialize, Deserialize};
use clap::Parser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reason {
  pub reason: String
}

impl Reason {
  pub fn new(reason: String) -> Self {
    Reason {
      reason
    }
  }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reason({})", self.reason)
    }
}


// ADT
// BuildFinished | CompilerMessage
//Also need to be able to filter by file and error level. Eg. no warnings or only errors in x.rs
//We also need to limit the number of errors displayed.

enum CompilerOutput {
  Status(BuildFinished),
  Message(CompilerMessage)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuildFinished {
  reason: String,
  success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessage {
  pub target: CompilerMessageTarget,
  pub message: CompilerMessageMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessageTarget {
  pub name: String,
  pub src_path: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessageMessage {
  pub rendered: String,
  pub code: Option<CompilerMessageCode>,
  pub level: String,
  pub message: String,
  pub spans: Vec<CompilerMessageSpan>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessageCode {
  code: String,
  explanation: Option<String>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessageSpan {
  byte_end: usize,
  byte_start: usize,
  column_end: u32,
  column_start: u32,
  expansion: Option<String>,
  pub file_name: String,
  is_primary: bool,
  label: Option<String>,
  line_end: u32,
  line_start: u32,
  suggested_replacement: Option<String>,
  suggestion_applicability: Option<String>,
  text: Vec<CompilerMessageSpanText>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerMessageSpanText {
  highlight_end: u32,
  highlight_start: u32,
  text: String
}

#[derive(Parser)]
#[clap(author, version, about)]
#[derive(Debug, Clone)]
pub struct Cli {

  #[clap(long, value_parser)]
  pub errors: u8,

  #[clap(long, value_parser)]
  pub filter: Option<String>
}

