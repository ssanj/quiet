use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug, Clone)]
pub struct Cli {

  /// The number of items to show. This includes errors and warnings to display. By default only errors are shown. Use --show-warnings to include warnings. Values range from 0 to 255.
  #[clap(long, value_parser)]
  pub items: u8,

  /// Flag to include warnings in the output
  #[clap(long, value_parser)]
  #[arg(default_value_t = false)]
  pub show_warnings: bool,

  /// The file (if any) to filter on. Example: --file-filter main.rs
  #[clap(long, value_parser)]
  pub file_filter: Option<String>
}

