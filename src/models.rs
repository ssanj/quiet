use serde::{Serialize, Deserialize};

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
  target: CompilerMessageTarget,
  message: CompilerMessageMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerMessageTarget {
  name: String,
  src_path: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerMessageMessage {
  rendered: String,
  code: Option<CompilerMessageCode>,
  level: String,
  message: String,
  // spans: Vec<CompilerMessageSpan>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerMessageCode {
  code: String,
  explanation: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerMessageSpan {
  byte_end: usize,
  byte_start: usize,
  column_end: u32,
  column_start: u32,
  expansion: String,
  file_name: String,
  is_primary: bool,
  label:String,
  line_end: u32,
  line_start: u32,
  suggested_replacement: String,
  suggestion_applicability: String,
  text: Vec<CompilerMessageSpanText>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerMessageSpanText {
  highlight_end: u32,
  highlight_start: u32,
  text: String
}
