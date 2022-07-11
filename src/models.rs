use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reason {
  reason: String
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
