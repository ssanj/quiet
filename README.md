![GitHub Release](https://img.shields.io/github/v/release/ssanj/quiet)

# Quiet

When refactoring or working in some intermediate state, Cargo can end up spamming you with a lot of errors or warnings that you don't really care about. It would be nice to filter out these messages until you actually care about them.

Quiet lets you do this by only showing the number of errors you want. Quiet can also limit errors to only a file you specify.


## Usage

```
Reduce Cargo's compiler information output

Usage: quiet [OPTIONS] --items <ITEMS>

Options:
      --items <ITEMS>                The number of items to show. Values range from 0 to 255. This includes errors and warnings to display. By default only errors are shown. Use --show-warnings to include warnings
      --show-warnings                Flag to include warnings in the output
      --file-filter <FILE_FILTER>    The file (if any) to filter on. Matches the file at the end of the path so you don't have to specify a full path. Example: --file-filter main.rs
      --filtered-out <FILTERED_OUT>  Strings to filter out from the output. Note, this only applies to Strings quiet does not need to output. Example --filtered-out "libunwind: malformed"
  -h, --help                         Print help information (use `--help` for more detail)
  -V, --version                      Print version information
```

Cargo output should be passed to Quiet through the following format:

```
 cargo check -q --message-format json-diagnostic-rendered-ansi 2>&1
 ```

For example to show only a single error for a project, run the following from your project directory:

```
cargo check -q --message-format json-diagnostic-rendered-ansi 2>&1 | quiet --items 1
```

You can also use it while running tests:

```
cargo test --message-format json-diagnostic-rendered-ansi 2>&1 | quiet --items 1
```

You can use it with `cargo watch` as:

```
cargo watch -x 'test --message-format json-diagnostic-rendered-ansi 2>&1  | quiet --items 1'
```

Also look at the [qcompile](https://github.com/ssanj/quiet/blob/main/qcompile), [qcompile-test](https://github.com/ssanj/quiet/blob/main/qcompile-test) and [qrun-test](https://github.com/ssanj/quiet/blob/main/qrun-test) sample scripts in this repository.

Note: The above scripts use [watch](https://crates.io/crates/cargo-watch), which you can install with:

```
cargo install cargo-watch
```

## Installation

### Building from source

Build Quiet with:

```
cargo build --release
```

Then copy `target/release/quiet` to a directory on your PATH.


### Downloading a release

Download the latest release for your platform from the [GitHub releases page](https://github.com/ssanj/quiet/releases).

Make it executable with:

`chmod +x <QUIET_EXEC>`

Copy executable to a directory on your path.

