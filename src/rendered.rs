use std::hash::{Hash, Hasher};
use super::CompilerMessage;

#[derive(Debug, Clone, Eq)]
pub struct Rendered {
  pub items: CompilerMessage
}

impl Rendered {
  pub fn new(items: CompilerMessage) -> Self {
    Self {
      items
    }
  }
}

impl PartialEq for Rendered {

  fn eq(&self, other: &Self) -> bool {
      self.items.message.rendered == other.items.message.rendered
  }
}

impl Hash for Rendered {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write(self.items.message.rendered.as_bytes())
  }
}
