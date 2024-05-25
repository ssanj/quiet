use crate::CompilerMessage;

pub fn by_filename(file_to_show_errors_for: Option<String>, matched: Vec<CompilerMessage>) -> Vec<CompilerMessage> {
  match file_to_show_errors_for {
    Some(file_name_filter) => {
      matched
        .into_iter()
        .filter(|compiler_message|{
          let filter_matches =
            compiler_message
              .message
              .spans
              .iter()
              .filter(|span|{
                span
                  .file_name
                  .ends_with(&file_name_filter)
                });

          !filter_matches.collect::<Vec<_>>().is_empty()
        })
        .collect::<Vec<_>>()
    },
    None => matched
  }
}
