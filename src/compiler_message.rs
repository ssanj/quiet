use serde::{Serialize, Deserialize};


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
  pub file_name: String,
  is_primary: bool,
  label: Option<String>,
}
