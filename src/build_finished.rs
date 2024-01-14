use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuildFinished {
  reason: String,
  success: bool
}
