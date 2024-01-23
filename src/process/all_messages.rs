use crate::CompilerMessage;

#[derive(Debug, Clone, Default)]
pub struct AllMessages {
  pub compiler_messages: Vec<CompilerMessage>,
  pub stdout_lines: Vec<String>,
  pub errors: Vec<String>
}

impl AllMessages {

  pub fn new() -> Self {
    Default::default()
  }

  pub fn add_compiler_message(&mut self, cm: CompilerMessage) {
    self.compiler_messages.push(cm)
  }

  pub fn add_stdout_line(&mut self, line: String) {
    self.stdout_lines.push(line)
  }

  pub fn add_error(&mut self, error: String) {
    self.errors.push(error)
  }
}

