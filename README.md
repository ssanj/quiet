# Quiet

When refactoring or working in some intermediate state, Cargo can end up spamming you with a lot of errors or warnings
that you don't really care about. It would be nice to filter out these messages until you actually care about them.

Quiet lets you do this by only showing the number of errors you want. Quiet can also limit errors to only a file you
specify.


## Usage

```
quiet 0.1.11

Reduce Cargo's compiler information output

Usage: quiet [OPTIONS] --errors <ERRORS>

Options:
      --errors <ERRORS>            The number of errors and/or warnings to display
      --show-warnings              Flag to include warnings in the output
      --file-filter <FILE_FILTER>  The file (if any) to filter on Example: --file-filter main.rs
  -h, --help                       Print help information
  -V, --version                    Print version information

```

Cargo output should be passed to Quiet through the following format:

```
 cargo check -q --message-format json-diagnostic-rendered-ansi
 ```

For example to show only a single error for a project, run the following from your project directory:

```
cargo check -q --message-format  json-diagnostic-rendered-ansi | quiet --errors 1
```

You can also use it while running tests:

```
cargo test -q --message-format  json-diagnostic-rendered-ansi | quiet --errors 1
```

You can use it with `cargo watch` as:

```
cargo watch -x 'test -q --message-format json-diagnostic-rendered-ansi | quiet --errors 1'
```

## Building

Build Quiet with:

```
cargo build --release
```

