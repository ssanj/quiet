use crate::CompilerMessage;

pub enum CompilerMessageDecodingStatus {
  DecodedCompilerMessage(CompilerMessage),
  StdOutLine(String),
  NoCompilerMessage
}
