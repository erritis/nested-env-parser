[![crates.io][crates-badge]][crates-url]
[![documentation][docs-badge]][docs-url]
[![MIT License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/nested-env-parser.svg
[crates-url]: https://crates.io/crates/nested-env-parser
[docs-badge]: https://docs.rs/nested-env-parser/badge.svg
[docs-url]: https://docs.rs/nested-env-parser
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

# nested-env-parser

Nested Env Parser is a crate for getting the final value of a string with nested environment variables.

## installation

Install using cargo:

```no_run,ignore
cargo install nested-env-parser
```

## Usage

### On Unix

```rust
use clap::Parser;
use nested_env_parser::Env;

#[derive(Clone, Debug, Parser)]
struct Opts {
    #[clap(env)]
    value_with_env: Env,
}

fn main() {
    std::env::set_var("VALUE1", "Hello,");
    std::env::set_var("VALUE2", "world");
    std::env::set_var("VALUE_WITH_ENV", "$VALUE1 ${VALUE2}!");

    let opts = Opts::parse();

    let value: &str = &opts.value_with_env;

    assert_eq!("Hello, world!", value);
}
```
### On Windows

```rust
use clap::Parser;
use nested_env_parser::Env;

#[derive(Clone, Debug, Parser)]
struct Opts {
    #[clap(env)]
    value_with_env: Env,
}

fn main() {
    std::env::set_var("VALUE1", "Hello");
    std::env::set_var("VALUE2", "world");
    std::env::set_var("VALUE_WITH_ENV", "%VALUE1%, %VALUE2%!");

    let opts = Opts::parse();

    let value: &str = &opts.value_with_env;

    assert_eq!("Hello, world!", value);
}
```

Current version: 1.0.0

License: MIT
