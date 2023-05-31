//! Nested Env Parser is a crate for getting the final value of a string with nested environment variables.
//!
//! # installation
//!
//! Install using cargo:
//!
//! ```no_run,ignore
//! cargo install nested-env-parser
//! ```
//!
//! # Usage
//! 
//! ## On Unix
//!
//! ```
//! use clap::Parser;
//! use nested_env_parser::Env;
//!
//! #[derive(Clone, Debug, Parser)]
//! struct Opts {
//!     #[clap(env)]
//!     value_with_env: Env,
//! }
//!
//! fn main() {
//!     std::env::set_var("VALUE1", "Hello,");
//!     std::env::set_var("VALUE2", "world");
//!     std::env::set_var("VALUE_WITH_ENV", "$VALUE1 ${VALUE2}!");
//!
//!     let opts = Opts::parse();
//!
//!     assert_eq!("Hello, world!", &opts.value_with_env);
//! }
//! ```
//! ## On Windows
//! 
//! ```no_run
//! use clap::Parser;
//! use nested_env_parser::Env;
//!
//! #[derive(Clone, Debug, Parser)]
//! struct Opts {
//!     #[clap(env)]
//!     value_with_env: Env,
//! }
//!
//! fn main() {
//!     std::env::set_var("VALUE1", "Hello");
//!     std::env::set_var("VALUE2", "world");
//!     std::env::set_var("VALUE_WITH_ENV", "%VALUE1%, %VALUE2%!");
//!
//!     let opts = Opts::parse();
//!
//!     assert_eq!("Hello, world!", &opts.value_with_env);
//! }
//! ```
#![warn(missing_docs)]

use std::ops::Deref;
use envmnt::{ExpandOptions, ExpansionType};

/// String with nested environment variables
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env(String);

impl Env {

    /// Create new Env
    pub fn new(value: &str) -> Self {
        value.into()
    }
}


impl From<&str> for Env {
    fn from(value: &str) -> Self {
        let mut options = ExpandOptions::new();

        options.expansion_type = match std::env::consts::FAMILY {
            "unix" => Some(ExpansionType::Unix),
            "windows" => Some(ExpansionType::Windows),
            _ => Some(ExpansionType::All)
        };
        let result = envmnt::expand(value, Some(options));
        Env(result)
    }
}

impl Deref for Env {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Env
where
    T: ?Sized,
    <Env as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl PartialEq<String> for Env {
    fn eq(&self, other: &String) -> bool {
        *self.deref() == *other
    }
}
impl PartialEq<Env> for String {
    fn eq(&self, other: &Env) -> bool {
        *self == *other.deref()
    }
}

impl PartialEq<str> for Env {
    fn eq(&self, other: &str) -> bool {
        *self.deref() == *other
    }
}
impl PartialEq<Env> for str {
    fn eq(&self, other: &Env) -> bool {
        *self == *other.deref()
    }
}