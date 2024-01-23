use itertools::Itertools;

use crate::Rendered;
use crate::CompilerMessage;
use super::level_status::LevelType;

pub fn by_number(mut filtered_by_level: Vec<LevelType>, items_to_show: usize, show_warnings: bool) -> Vec<CompilerMessage> {
  if !show_warnings {
    // Errors only
    filtered_by_level
      .into_iter()
      .filter_map(|lt| {
        match lt {
          LevelType::ErrorLevel(cm) => Some(cm),
          _                         => None,
        }
      })
      .take(items_to_show)
      .collect()
  } else {
      // Both errors and warnings
      // Sort with errors first, then warnings
      filtered_by_level
      .sort_by_key(|lt|{
        match lt {
          LevelType::ErrorLevel(_)   => 0,
          LevelType::WarningLevel(_) => 1
        }
      });

      // The warnings returned by the Cargo JSON have duplicate elements.
      // We convert them to Rendered to allow us to remove duplicated with the same rendered output.
      filtered_by_level
        .into_iter()
        .filter_map(|lt|{
          match lt {
            LevelType::ErrorLevel(cm)   => Some(Rendered::new(cm)),
            LevelType::WarningLevel(cm) => {
              if cm.message.message.contains("warning emitted") { // Also remove messages that say "warning emitted"
                None
              } else {
                Some(Rendered::new(cm))
              }
            },
          }
        })
        .unique() // Removes duplicates
        .map(|r| r.items)
        .take(items_to_show)
        .collect()
  }
}
