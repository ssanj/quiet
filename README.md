# Quiet

Reduce the number of errors displayed by cargo. This is useful when refactoring or working in some intermediate state
where there are many errors or warnings, which you mostly want to ignore


## Usage

```
quiet 0.1.3

USAGE:
    quiet [OPTIONS] --errors <ERRORS>

OPTIONS:
        --errors <ERRORS>
        --filter <FILTER>
    -h, --help               Print help information
    -V, --version            Print version information
```

Cargo output should be passed to Quiet through:

```
 cargo check -q --message-format json-diagnostic-rendered-ansi
 ```

For example to show only a single error for a project, run:

```
 cargo check -q --message-format  json-diagnostic-rendered-ansi | quiet --errors 1
 ```
