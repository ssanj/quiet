use crate::CompilerMessage;

#[derive(Debug, Clone, PartialEq)]
pub enum LevelType {
  ErrorLevel(CompilerMessage),
  WarningLevel(CompilerMessage),
}

pub struct LevelStatus {
  pub errors: bool,
  pub warnings: bool
}

impl LevelStatus {
  fn copy_errors(self, new_errors: bool) -> Self {
    Self {
      errors: new_errors,
      warnings: self.warnings
    }
  }

  fn copy_warnings(self, new_warnings: bool) -> Self {
    Self {
      errors: self.errors,
      warnings: new_warnings
    }
  }
}

pub struct LevelInfo {
  pub status: LevelStatus,
  pub level_types: Vec<LevelType>
}


pub fn by_level(filtered_match: Vec<CompilerMessage>) -> LevelInfo {
  let filtered_by_level: Vec<LevelType> = filter_by_level(filtered_match);
  let level_status = get_level_status(&filtered_by_level);
  LevelInfo {
    status: level_status,
    level_types: filtered_by_level
  }
}


fn filter_by_level(filtered_match: Vec<CompilerMessage>) -> Vec<LevelType> {
  filtered_match
    .into_iter()
    .filter_map(|i| {
      let level = &i.message.level;
      match level.as_str() {
        "error"   => Some(LevelType::ErrorLevel(i)),
        "warning" => Some(LevelType::WarningLevel(i)),
        _         => None
      }
    })
    .collect()
}


fn get_level_status(filtered_by_level: &[LevelType]) -> LevelStatus {
  let init =
    LevelStatus {
      errors: false,
      warnings: false,
    };

  let result =
    filtered_by_level
      .iter()
      .fold(init, |acc, v| {
        match v {
          LevelType::ErrorLevel(_) => {
            if !acc.errors {
              acc.copy_errors(true)
            } else {
              acc
            }
          },
          LevelType::WarningLevel(_) => {
            if !acc.warnings {
              acc.copy_warnings(true)
            } else {
              acc
            }
          },
        }
      });

  result
}

