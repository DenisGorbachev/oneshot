## Guidelines

### General

You are a senior Rust software architect.

- Think deeply and make detailed plans before writing code.
- Write high-quality, production-ready, generic, reusable code.

#### Principles

Write code that minimizes losses:

- [Avoid data loss](#avoid-data-loss).
- [Minimize hardcoded data](#minimize-hardcoded-data).
- Minimize memory consumption.
- Minimize execution time.

##### Avoid data loss

- Don't use panicking fns (instead, use checked fns that return a `Result`).
- Don't delete data unless spec explicitly requires it.

##### Minimize hardcoded data

- Don't hardcode values (accept args instead).
- Choose carefully between accepting a param and defining a const:
  - Defs:
    - Params are execution details (user may want to change them).
    - Consts are impl details (user would never want to change them).
  - Examples:
    - Params:
      - Cache TTL.
      - Config path.
    - Consts:
      - Table name.
      - Keyspace name.
  - Recommendations:
    - When in doubt, prefer accepting a param instead of defining a const.

#### Development workflow

- Run `mise run fix` before every commit.
- Don't maintain backward compatibility.
- Don't add comments.
- Don't edit files in `.agents`.
- Don't run `git diff` or `git status` solely to review your own work at end of a turn.
- If a later instruction overrides an earlier instruction, follow later instruction.
- If I explicitly ask to update code in a way that deviates from spec, update code, spec.
- If you notice unexpected edits:
  - Then:
    - Keep them.
    - Don't mention them.
- If you see a code item or spec requirement that is unnecessary or incorrect, tell me.
- If you have to apply a workaround:
  - Then:
    - Add a comment next to workaround explaining why it is necessary.
    - Mention workaround in your final report.
- If unexpected behavior impedes progress but is not a blocker (for example: domain is unavailable, program is unavailable, available memory or disk space is too low, cmd runs for an unexpectedly long time or consumes an unexpected amount of resources), mention it in your final msg.
- If task is technically possible but would result in low-quality code:
  - Then:
    - If there is a clearly better alternative, implement it; else: explain why requested code should not be written.
  - Examples:
    - A task to write `impl From<Foo> for Bar` where `Foo` can't actually be infallibly converted to `Bar` (conversion would require calling `unwrap`, which is bad) — in this case, you should write `impl TryFrom<Foo> for Bar` and reply with "Foo can't be infallibly converted to Bar, so I implemented a fallible conversion instead".
    - A task to write a trait impl that only returns an error — in this case, you should not write trait impl but reply with "trait X can't be implemented for Foo because ..."
- If a sentence starts with "Idea: ":
  - Then:
    - Evaluate it thoroughly.
    - If you agree, implement it; else: explain why you didn't implement it and brainstorm solutions.

#### Review workflow

- Output a full list of [findings](#finding) (not a shortlist).
- If there are no findings, start your reply with "No findings".
- For each item in an ordered reply to your review:
  - If item is "+", reconsider finding, apply best fix.
    - Else:
      - If item matches "+ {number}", apply proposed fix {number}.
        - Else:
          - If item is "-", don't apply fixes; else: respond normally (keep `ctid` in your response).
- If no actionable items remain in thread identified by a specific `ctid`, drop this `ctid` from your response.

#### Debugging workflow

- Improve error handling, so that root cause is clearly visible.

#### Subagents

- When spawning a code review subagent: use fresh ctx (not inherited).

#### Cmds

- Use `fd` and `rg` instead of `find` and `grep`.
- Set cmd-execution tool call’s timeout param and `yield_time_ms` to at least 300000 ms for cmds: `mise run fix`, `cargo build`, `git commit`.

#### Recommended crates

- `errgonomic` for error handling.
- `strum` for enum derives.
- `subtype` for newtypes.
- `tempfile` for creating temp dirs or files.

#### Files

- File name must match name of primary item in this file (for example: a file with `struct User` must be named `user.rs`).
- Trait impls must be in same file as target type (for example: put `impl TryFrom<...> for User` in same file as `struct User`, which is `user.rs`).

#### Modules

- Don't use `mod.rs`; use module files with submodules in folder with same name (for example: `user.rs` with submodules in `user` folder).
- When creating a new module, attach it with a `mod` declaration followed by a `pub use` glob declaration. The parent module must re-export all items from child modules. For example:
  ```rust
  fn foo() {}

  mod my_module_name;
  pub use my_module_name::*;
  ```
- Place `mod` and `pub use` declarations at end of file (after code items).
- When importing items that are defined in current crate, import them directly from crate root. For example:
  ```rust
  use crate::foo;
  ```
- Prefer short item paths over long item paths (use a `use` statement), unless it's necessary for disambiguation. For example:
  - Good:
    ```rust
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Good (`serde` and `rkyv` prefixes are necessary for disambiguation):
    ```rust
    use clap::ValueEnum;

    #[derive(ValueEnum, From, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Bad (`clap` and `serde` prefixes are not necessary for disambiguation because their trait names are unique in this module):
    ```rust
    #[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
- If you need error and result types from `std`, prefer short paths.
  - Examples:
    - `use std::io;`: `io::Result`, `io::Error`.
    - `use std::fmt;`: `fmt::Result`, `fmt::Error`.

#### Visibility

- Items:
  - Prefer `pub` visibility over `pub(crate)` or private visibility.
- Fields:
  - If a struct is a refinement of its fields:
    - Then:
      - Fields must be private.
      - Fns that construct or deserialize struct or mutate its fields must preserve invariant.
    - Else: its fields must be `pub`.

#### Consts

- Define consts only for values used in multiple places (prefer inline values).
- Put consts in `src/constants.rs`.

#### Types

- Always use most specific types (enforce semantic difference through syntactic difference):
  - Use types from existing crates.
    - Use types from `url` crate instead of `String` for URL-related values.
    - Use types from `time` crate instead of `String` or `u64` for datetime-related values.
    - Use types from `phonenumber` crate instead of `String` for phone-related values.
    - Use types from `email_address` crate instead of `String` for email-related values.
    - Use types from `core::num` module that are prefixed with `NonZero` for values that must be non-zero.
  - Search for other existing crates if you need specific types.
  - If you can't find existing crates, define newtypes.
- Every `struct`, `enum`, `union` must be in a separate file (except for error types that implement `Error`).
  - Error types that implement `Error` must be in same files as fns that return them.
- Prefer attaching types as child modules to `src/types.rs`.

#### Fns

- Prefer weakest sufficient trait bound for inputs and associated types (`FnOnce` over `FnMut` over `Fn`, `PartialOrd` over `Ord`, `PartialEq` over `Eq`).
- Implement proper error handling using macros from `errgonomic` crate instead of `unwrap` or `expect` (in normal code and in tests).
  - Use `expect` only in exceptional cases where you can prove that it always succeeds, and provide proof as first arg to `expect` (proof must start with "always succeeds because").
- Prefer streams and iterators:
  - Guidelines for inputs:
    - If fn uses methods that are available only for a specific collection type, prefer taking a specific collection type as input; else: prefer taking an `impl Stream` or `impl IntoIterator` as input.
  - Guidelines for outputs:
    - If fn return type is naturally an iterator (for example: fn returns output of a `map` or `filter`), prefer returning an `impl Iterator` as output (there's no need to collect into `Vec`); else: prefer returning a specific collection type as output.
  - Examples:
    - Good:
      ```rust
      /// Good: fn uses only generic `Iterator` methods.
      /// Good: fn naturally returns an iterator.
      pub fn filter_non_empty_strings<'a>(inputs: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
          inputs.into_iter().filter(|i| i.is_empty().not())
      }

      /// Good: `Vec`-specific `extend_from_slice` requires `Vec` input.
      fn extend_args(mut args: Vec<String>, extra_args: &[String]) -> Vec<String> {
          args.extend_from_slice(extra_args);
          args
      }
      ```
    - Bad:
    - ```rust
      /// Bad: needlessly converts a `Vec` into an iterator, then collects into a `Vec`.
      pub fn filter_non_empty_strings(inputs: Vec<&str>) -> Vec<&str> {
          inputs
              .into_iter()
              .filter(|i| i.is_empty().not())
              .collect::<Vec<_>>()
      }

      /// Bad: forces caller to collect strings into a `Vec`, restricting input types and adding allocations.
      pub fn bar(inputs: Vec<String>) -> Vec<String> {}
      ```
- Prefer implementing and using `From` or `TryFrom` for conversions between types (instead of converting in-place).
- Don't use early-return fast-path guards for empty vectors, iterators, streams (i.e.: don't use `if items.is_empty() { return ...; }`).
- Use destructuring assignment for tuple args, for example: `fn try_from((name, parent_key): (&str, GroupKey)) -> ...`.
- Use iterators instead of for loops. For example:
  - Good:
    ```rust
    use errgonomic::{handle_iter, ErrVec};
    use core::num::ParseIntError;
    use thiserror::Error;

    // Good: iterator pipeline with fallible mapping and error handling.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseNumbersError> {
        use ParseNumbersError::*;
        let iter = inputs.into_iter().map(|s| s.as_ref().trim().parse::<u64>());
        Ok(handle_iter!(iter, InvalidInput))
    }

    #[derive(Error, Debug)]
    pub enum ParseNumbersError {
        #[error("failed to parse {len} numbers", len = source.len())]
        InvalidInput { source: ErrVec<ParseIntError> },
    }
    ```
  - Bad:
    ```rust
    use core::num::ParseIntError;

    // Bad: manual loop and mutable accumulator.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseIntError> {
        let mut out = Vec::new();
        for s in inputs {
            let n = s.as_ref().trim().parse::<u64>()?;
            out.push(n);
        }
        Ok(out)
    }
    ```
- If fn has a clear receiver (`self`, `&self`, `&mut self`), implement it as an associated fn; else: implement it as a standalone free fn.
- Add a local `use` statement for enums to minimize code size. For example:
  - Good:
    ```rust
    pub fn apply(op: GroupsOp) {
        use GroupsOp::*;
        match op {
            InsertOne(_) => {}
            UpdateOne(_, _) => {}
            DeleteOne(_) => {}
        }
    }
    ```
  - Bad:
    ```rust
    pub fn apply(op: GroupsOp) {
        match op {
            GroupsOp::InsertOne(_) => {}
            GroupsOp::UpdateOne(_, _) => {}
            GroupsOp::DeleteOne(_) => {}
        }
    }
    ```
- Simplify call-site code by accepting `impl Into`. For example:
  - Good:
    ```rust
    pub fn foo(input: impl Into<String>) {
        let input = input.into();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.into()` when passing input arg.
    pub fn foo(input: String) {}
    ```
- Provide additional flexibility at call sites by accepting `&impl AsRef` or `&mut impl AsMut` (e.g.: both `PathBuf` and `Config` may implement `AsRef<Path>`). For example:
  - Good:
    ```rust
    pub fn bar(input: &mut impl AsMut<String>) {
        let input = input.as_mut();
    }

    pub fn baz(input: &impl AsRef<str>) {
        let input = input.as_ref();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.as_mut()` when passing input arg.
    pub fn bar(input: &mut String) {}

    /// Bad: caller may have to call `.as_ref()` when passing input arg.
    pub fn baz(input: &str) {}
    ```
- Prefer `.map()` instead of `match` when you need to modify value in `Option` or `Result`. For example:
  - Good:
    ```rust
    use core::str::FromStr;
    use core::num::ParseIntError;

    impl FromStr for UserId {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map(Self::new)
        }
    }
    ```
  - Bad:
  ```rust
  use core::str::FromStr;
  use core::num::ParseIntError;

  impl FromStr for UserId {
      type Err = ParseIntError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
          // Bad: uses more code to express same idea.
          match s.parse::<u64>() {
              Ok(value) => Ok(Self::new(value)),
              Err(error) => Err(error),
          }
      }
  }
  ```
- Use `Self` instead of type name in `impl` items. For example:
  - Good:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          Self::new(duration.as_secs())
      }
  }
  ```
  - Bad:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          UnixTimestamp::new(duration.as_secs())
      }
  }
  ```
- Prefer short method syntax over long trait-path syntax (prefer `value.method(arg)` over `Trait::method(value, arg)`).
- Generic helper fns must be in `src/functions` (one file per fn).
- If Clippy reports `too_many_arguments`:
  - Then:
    - Don't use tuples to silence lint.
    - Consider refactoring code for better separation of concerns.

#### Struct derives

- Derive `new` from `derive_new` crate for types that need `fn new`.
- If struct derives `Getters`:
  - Then:
    - For each field:
      - If its type implements `Copy`, it must have a `#[getter(copy)]` annotation.
  - Examples:
    - Good (`username` has no `#[getter(copy)]` because `String` doesn't implement `Copy`, but `age` has it because `u64` implements `Copy`):
      ```rust
      #[derive(Getters, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
      pub struct User {
        username: String,
        #[getter(copy)]
        age: u64,
      }
      ```

#### Setters

- Use setters that take `&mut self` instead of setters that take `self` and return `Self` (because passing `foo: &mut Foo` is more efficient than passing `foo: Foo` and returning `Foo` through call stack).

#### Enums

- When writing code related to enums, bring variants into scope with a `use Enum::*;` statement at top of file or fn (prefer top of file for data enums and top of fn for error enums).

#### Arithmetic

- Don't use impls of traits `core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Shl, ShlAssign, Shr, ShrAssign}` or their operators unless they don't panic or silently overflow.
- Write and use arithmetic trait impls that don't panic or silently overflow.
- Prefer `checked` versions of arithmetic ops.
- Every call to an `overflowing`, `saturating`, `wrapping` version must have a single-line comment above it that starts with "SAFETY: " and describes why calling this version is safe in this specific case.
- Use items from `num` crate if necessary (for example: to implement a fn that calls arithmetic methods on a generic type).

#### Index access

- Never use operators: `[], []=`.
- Never use traits: `core::ops::{Index, IndexMut}`.
- If you are sure that `get` or `get_mut` will never panic, use `expect` with a proof msg (as described in [Fns](#fns)).

#### Test fn

A fn marked with `#[test]` or `#[tokio::test]`.

- Must return a `Result`.
- Must implement proper error handling via `errgonomic` crate.
- Should use macros from `assertables` crate.
  - Should use `assert_infix` instead of `assert_gt`, `assert_ge`, `assert_lt`, `assert_le`, `assert_eq`.

#### Macros

- In `macro_rules!` expansions:
  - Use full paths without a leading `::` for foreign types (e.g.: `std::sync::Arc`), except those in standard prelude.
  - Use `$crate` paths for types defined in this crate (e.g.: `$crate::MyType`).
- If you see similar code in different places, replace similar code with a shared macro.
- If last arg of a macro arm is variadic (e.g.: `$($item:ident),*`), add a trailing `$(,)?`; else: don't add a trailing `$(,)?`.

#### Shell

- Prefer short options for cmds used in tool calls.

#### mise.toml

- Each `env` map value:
  - If it should be overridable by the user:
    - Then: it must be a map with `default` key (e.g. `PORT = { default = "3000" }`)
    - Else: it must be a specific value (e.g. `SHUCK_CACHE_DIR = "{{config_root}}/.cache"`)

#### Cargo.toml

- Don't define pkg features with only a single optional dep (such features are already defined by Cargo automatically).
- Use `cargo add` to add deps.
- When adding a dep from crates.io:
  - If pkg is [publishable](#publishable-pkg):
    - Then:
      - Run `cargo add {dependency}@{version}`.
        - Patch component of `{version}` must be 0.
      - Try `cargo update -p {dependency} --precise {version}` to lock that exact version.
        - If dep constraints prevent locking that version:
          - Then:
            - Keep version resolved by Cargo.
            - Add a comment in `Cargo.toml` explaining constraints.
    - Else:
      - Run `cargo add {dependency}` without `{version}`.
- When adding a dep in a workspace:
  - Add it to top-level manifest first (`workspace.dependencies`).
- When adding a dep for a workspace member:
  - Run `cargo add {dependency}` without `{version}` (Cargo will set `workspace = true`).
- When adding a new workspace member: add it to `packages` dir unless specified otherwise.

#### Code style

- Don't enforce a line len limit when writing code, comments, docs.

#### Chat thread id

- Must be a string.
- Must contain at least 3 characters.
- Must contain only uppercase characters.

Examples:

- `RVC`.
- `AKE`.
- `LMY`.

Notes:

- Should match thread topic.

#### Chat thread id heading

A level-3 Markdown heading that contains only a [chat thread id](#chat-thread-id).

Examples:

- `### RVC`.
- `### AKE`.
- `### LMY`.

#### Finding

- Must be formatted as `### {ctid}\n\n[{priority}] {title}. {body} ({references}). Proposed fixes: {fixes}`.
  - `ctid` must be a [chat thread id](#chat-thread-id).
  - `priority` must be one of `P0`, `P1`, `P2`, `P3`.
  - `references` must be a comma-separated list of `reference` values.
  - `reference` must be formatted as `{path}:{line}`.
  - `path` must be a file path relative to your working dir.
  - `line` must be first line of relevant code or text block.
  - `fixes` must be one of:
    - If there is at least one proposed fix, "\n\n" and a nested Markdown list of fixes where each fix must have format `{number}. {description}` (numbers should start from 1 for each list of fixes); else: exact text "none."

#### Publishable pkg

A pkg that has a remote whose name contains `public` or `pre-public` and ends with `template`.

### Guidelines for `serde`

#### Reqs

- Every input data type must derive `Serialize`, `Deserialize`.
- Every `Option`-wrapped field must have attr:
  - `#[serde(skip_serializing_if = "Option::is_none")]`.
- Every `OffsetDateTime` field must have attr:
  - `#[serde(with = "time::serde::rfc3339")]`.
- Every `Option<OffsetDateTime>` field must have attr:
  - `#[serde(with = "time::serde::rfc3339::option")]`.

#### Notes

- Prefer `serde_with` to reduce code size by avoiding custom `Serialize`/`Deserialize` impls.

### Guidelines for `subtype`

- Define newtypes as ordinary tuple structs with explicit `From` / `TryFrom` impls.
- Use `SerializeTransparent` derive from `subtype` for a refined newtype that must serialize identically to its inner field while using Serde's `try_from` container attr for validated deserialization.
- Macro calls that begin with `subtype` (for example: `subtype!` and `subtype_string!`) are legacy APIs that expand to newtypes.
  - Don't use them in new code because their checker and preprocessor concepts have been superseded by explicit conversion impls.

### Guidelines for `clap`

#### Reqs

- For each enum in project:
  - If enum has only unit variants and doesn't implement `Error`, it must derive `ValueEnum` with `#[value(rename_all = "kebab-case")]`.
- For each field in a type that derives `Parser`:
  - If this field's type is local, this type must implement `FromStr`.

### CLI guidelines

#### Deps

- `clap` (features: at least "derive", "env").
- `tokio` (features: at least "macros", "rt", "rt-multi-thread").
- `errgonomic`.
- `thiserror`.

#### File layout and required items

##### File `src/main.rs`

- Must define a `main` entry point.
- Must define a `verify_cli` test for top-level command exactly as in example below (with `debug_assert`).

Example:

```rust
use clap::Parser;
use errgonomic::exit_result;
use my_crate_name::Command;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Command::parse();
    let result = args.run().await;
    exit_result(result)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}
```

##### File `src/command.rs`

- Must define a [command-like struct](#command-like-struct) named `Command`.
  - Must have attr:
    - `#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]`.
- Must define a [subcommand-like enum](#subcommand-like-enum) named `Subcommand`.

Example:

```rust
use std::process::ExitCode;
use Subcommand::*;
use errgonomic::map_err;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommand {
    Print(PrintCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            subcommand,
        } = self;
        match subcommand {
            Print(command) => map_err!(command.run().await, PrintCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run print command")]
    PrintCommandRunFailed { source: PrintCommandRunError },
}

mod print_command;

pub use print_command::*;
```

#### Defs

##### Command-like struct

A struct that contains fields for CLI args.

- Must have a name formed by concatenating all command names leading up to and including this command, followed by `Command` (see example above).
- Must have at least attrs:
  - `derive`:
    - Must contain at least:
      - `Parser` (`use clap::Parser`).
  - `command`:
    - Must contain at least:
      - `flatten_help = true`.
- If it is a top-level command, must be attached to `src/lib.rs`; else: must be attached to `src/command.rs`.
- For each field:
  - If field has a collection type (e.g.: `Vec`), it must have attr `num_args = 1..`.
- May contain a `subcommand` field annotated with `#[command(subcommand)]`.
- Must have a `pub async fn run` method.
  - Must return a `Result` with an `ExitCode` success value.
  - If struct contains a `subcommand` field, this method must match on `subcommand` and call selected command's `run` method.

Command example:

- Name: `DbDownloadYcombinatorStartupsCommand`.
- File: `src/command/db_download_ycombinator_startups_command.rs` (attached to `src/command.rs`).
- Shell command: `cargo run -- db download ycombinator-startups`.

##### Subcommand-like enum

An enum that contains variants for CLI subcommands.

- Must have a name formed by concatenating all command names leading up to and including this command, followed by `Subcommand` (see example above).
- Must have at least attrs:
  - `derive`:
    - Must contain at least:
      - `Subcommand` (`use clap::Subcommand`).
- Must be located in same file as its parent command struct.
- Each variant must be a tuple variant containing exactly one command.

Subcommand example:

- Name: `DbDownloadSubcommand`.
- File: `src/cli/db_command/db_download_command.rs` (same file as its parent `DbDownloadCommand`).

##### Proxy command-like struct

A [command-like struct](#command-like-struct) that has a `subcommand` field and calls `run` on each subcommand.

Proxy command example:

- Name: `DbCommand`.
- File: `src/command/db_command.rs` (attached to `src/command.rs`).

## Project files

### mise.toml

```toml
min_version = "2026.7.13"

[settings]
idiomatic_version_file_enable_tools = ["rust"]
task.output = "keep-order"

[tools]
node = "24.15.0"
deno = "1.46.1"
fnox = "1.33.1"
fd = "10.4.2"
"github:ewhauser/shuck" = "0.2.2"
"aqua:nushell/nushell" = "0.116.1"
"cargo:nu-lint" = "1.8.0"
"github:nushell/nufmt" = "0.1.6"
"aqua:rvben/rumdl" = "0.3.0"
"npm:@commitlint/config-conventional" = "19.6.0"
"npm:@commitlint/cli" = "19.6.0"
"npm:@commitlint/types" = "19.5.0"
"npm:skills" = "1.5.24"
"cargo:https://github.com/DenisGorbachev/cargo-insert-docs" = { version = "rev:9bccf15cc367a50d2652b0eaf5da7faf5929c666", crate = "cargo-insert-docs", locked = true }
"cargo:cargo-hack" = "0.6.33"
"cargo:cargo-nextest" = "0.9.145"
"cargo:cargo-expand" = "1.0.114"
"cargo:taplo-cli" = "0.10.0"
"cargo:sd" = "1.0.0"
"cargo:cargo-edit" = "0.13.13"

[hooks]
postinstall = { task = "git:install-hooks" }

[env]
SHUCK_CACHE_DIR = "{{config_root}}/.cache"

[tasks."build"]
run = "cargo build --workspace"

[tasks."check"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test"] }]

[tasks."check:ci"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test:ci"] }]

[tasks."test"]
depends = ["test:code", "test:docs"]

[tasks."lint"]
depends = ["lint:name", "lint:configs", "lint:code", "lint:code:style", "lint:shell", "lint:nu", "lint:docs", "lint:generated", "lint:reports"]

[tasks."lint:name"]
run = [{ task = "fix:name", args = ["--check"] }]

[tasks."lint:configs"]
depends = ["lint:configs:cargo", "lint:configs:fnox"]

[tasks."lint:configs:cargo"]
run = [{ task = "fix:cargo", args = ["--check"] }]

[tasks."lint:configs:fnox"]
run = [{ task = "fix:fnox" }]

[tasks."lint:code"]
run = "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

[tasks."lint:code:style"]
run = "cargo fmt --all -- --check"

[tasks."lint:shell"]
# Shuck 0.2.2 scans Git-ignored local files, so exclude them explicitly.
run = '''shuck --config "lint.source-paths = ['$HOME']" check --extend-exclude '*.local.*' .'''

[tasks."lint:docs"]
run = "rumdl check"

[tasks."lint:generated"]
run = [{ task = "md:gen", args = ["AGENTS.ts"] }, { task = "md:gen", args = ["README.ts"] }]

[tasks."test:code"]
depends = ["test:code:regular", "test:code:integration"]

[tasks."test:code:regular"]
run = "fnox --profile test exec --replace -- cargo nextest run --locked --workspace --all-features --no-tests warn --cargo-quiet --show-progress none --no-input-handler --status-level fail --final-status-level flaky --no-fail-fast"

[tasks."test:code:integration"]
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code:regular", args = ["--ignore-default-filter", "--test-threads", "1", "integration_tests::"] }]

[tasks."test:docs"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "cargo test --locked --workspace --doc --all-features --no-fail-fast --quiet"

[tasks."test:ci"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "fnox --profile test exec --replace -- cargo hack test --locked --workspace --feature-powerset --keep-going --no-fail-fast --quiet -- --skip integration_tests::"

[tasks."pre-commit"]
alias = "pre-merge-commit"
run = [{ task = "git:validate-commit" }]

[tasks."commit-msg"]
run = 'mise run --output interleave commitlint -- --edit "$@"'

[tasks."fix"]
depends = ["cargo:validate-config"]
run = [{ task = "fix:aux" }]

[tasks."fix:aux"]
depends = ["fix:configs", "fix:docs", "fix:agents", "fix:readme"]

[tasks."fix:configs"]
depends = ["fix:cargo", "fix:fnox"]

[tasks."fix:code"]
depends = ["fix:name", "fix:code:style", "fix:shell", "fix:nu"]

[tasks."fix:nu"]
run = [{ task = "nu", args = ["nufmt", "--shebang"] }]

[tasks."fix:shell"]
run = [{ task = "lint:shell", args = ["--fix"] }]

[tasks."fix:code:warnings"]
depends = ["fix:cargo"]
# second pass is needed because "cargo clippy --fix" exits with 0 even if some warnings remain
env = { __CARGO_FIX_YOLO = 'yeah' }
run = [
    "cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged",
    { task = "lint:code" },
]

[tasks."fix:code:style"]
# Run after `fix:code:warnings` because both tasks modify the same code files.
depends = ["fix:code:warnings"]
run = "cargo fmt --all"

[tasks."fix:docs"]
depends = ["fix:agents", "fix:readme"]
# use `rumdl check --fix` instead of `rumdl fmt` because `rumdl check --fix` exits with 1 if errors remain (since v0.1.0)
run = [{ task = "lint:docs", args = ["--fix"] }]

[tasks."fix:agents"]
# "fix:agents" depends on "fix:code" because it reads the code files
depends = ["fix:name", "fix:configs", "fix:code"]
run = [{ task = "gen:agents" }]

[tasks."gen:readme"]
run = [{ task = "md:gen", args = ["README.ts", "--fix"] }]

[tasks."gen:agents"]
run = [{ task = "md:gen", args = ["AGENTS.ts", "--fix"] }]

[tasks."commitlint"]
run = "commitlint --extends \"$(mise where npm:@commitlint/config-conventional)/node_modules/@commitlint/config-conventional/lib/index.js\""

[tasks."agent:docs:list"]
run = "[ -d .agents/docs ] && find .agents/docs -type f -print || true"
output = "interleave"
quiet = true
```

### fnox.toml

```toml
#:schema https://fnox.jdx.dev/schema.json

if_missing = "error"
env = "exec"

[providers]
keychain = { type = "keychain", service = "oneshot" }
pass = { type = "password-store", prefix = "oneshot/" }
age = { type = "age", recipients = [
    "age1sf4r4amev2svqr6llwg8hgtz9n7p5qdh7hh0mavcshzfrmgfduksnq3hql",
    "age1605gsnxpe536sprwccyumq74veg0g80u55n8ggems0t8deau6qdsfnq3m3"
] }
```

### Cargo.toml

```toml
[workspace]
resolver = "3"
members = ["packages/clust_ext", "packages/oneshot_aist", "packages/oneshot_cli", "packages/oneshot_common", "packages/oneshot_rust", "packages/oneshot_rust_anthropic", "packages/oneshot_utils", "packages/pseudorust", "packages/tasks"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.93.1"
publish = false

[workspace.metadata.details]
name = "oneshot"
title = "Oneshot"
readme = { generate = false }

[workspace.lints.rust]
redundant_imports = "deny"
unused_import_braces = "deny"
# unused_qualifications must not be "deny" because our code style has multiple `use Foo::*;`, and some macros (derive_more::Display, strum::Display, strum::EnumString) produce code with full qualifications
# unused_qualifications = "deny"

[workspace.lints.clippy]
# Rust 1.100 reports redundant fields inside derive macros: https://github.com/rust-lang/rust-clippy/issues/17525.
redundant_field_names = "allow"
absolute_paths = "deny"
arithmetic_side_effects = "deny"

[workspace.dependencies]
derive-new = "0.7.0"
derive_more = { version = "1.0.0", features = ["full"] }
fmt-derive = "0.1.2"
derive_builder = "0.20.0"
derive-getters = { version = "0.5.0", features = ["auto_copy_getters"] }
anyhow = "1.0.80"
proptest = "1.5.0"
clust = { git = "https://github.com/DenisGorbachev/clust", branch = "dev", features = ["hash"] }
tokio = { version = "1.43.0", features = ["macros", "fs", "net", "rt", "rt-multi-thread"] }
indoc = "2.0.5"
lazy_static = "1.5.0"
strum = { version = "0.26.3", features = ["derive"] }
clap = { version = "4.5.8", features = ["derive", "env"] }
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.120"
serde-xml-rs = "0.6.0"
reqwest = "0.12.5"
proc-macro2 = { version = "1.0.86", features = ["span-locations"] }
syn = { version = "2.0.68", features = ["full", "visit"] }
quote = { version = "1.0.36" }
env_logger = { version = "0.11.3" }
tokio-stream = { version = "0.1.15" }
clust_ext = { path = "./packages/clust_ext" }
save-load = { git = "https://github.com/DenisGorbachev/save-load", features = ["clap", "serde_json", "serde_yaml"] }
subtype = { git = "https://github.com/DenisGorbachev/subtype" }
helpful = { git = "https://github.com/DenisGorbachev/helpful" }
oneshot_common = { path = "./packages/oneshot_common" }
oneshot_utils = { path = "./packages/oneshot_utils" }
oneshot_rust = { path = "./packages/oneshot_rust" }
fs-err = { version = "2.11.0" }
itertools = { version = "0.13.0" }
bat = { version = "0.24.0", default-features = false, features = ["paging", "regex-fancy"] }
prettyplease = { version = "0.2.20" }
pulldown-cmark = { version = "0.11.0" }
throbber-widgets-tui = { version = "0.6.0" }
ratatui = { version = "0.27.0" }
tracing = { version = "0.1.40" }
tracing-subscriber = { version = "0.3.20", features = ["default", "local-time", "env-filter"] }
serde_yaml = { version = "0.9.34" }
time = { version = "0.3.47", features = ["macros", "formatting"] }
thiserror = { version = "1.0.61" }
constcat = { version = "0.5.0" }
homedir = { version = "0.3.3" }
directories = { version = "5.0.1" }
num-traits = { version = "0.2.19" }
tap = { version = "1.0.1" }
indexmap = { version = "2.2.6" }
rustc-hash = { version = "2.0.0" }
quick-xml = { version = "0.35.0", features = ["serialize"] }
duct = { version = "0.13.7" }
tempfile = { version = "3.10.1" }
lsp-types = { version = "0.97.0" }
#[profile.test.package.proptest]
#opt-level = 3
#
#[profile.test.package.rand_chacha]
#opt-level = 3

[profile.dev]
opt-level = 1
```

### packages/clust_ext/Cargo.toml

```toml
[package]
name = "clust_ext"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
clust = { workspace = true }
itertools = { workspace = true }
derive_more = { workspace = true }
derive-new = { workspace = true }
serde = { workspace = true, optional = true }

[lints]
workspace = true
```

### packages/oneshot_aist/Cargo.toml

```toml
[package]
name = "oneshot_aist"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive_more = { version = "2.0.1", features = ["full"] }
derive-new = { version = "0.7.0" }
derive-getters = { version = "0.5.0", features = ["auto_copy_getters"] }
fmt-derive = "0.1.2"
thiserror = { version = "2.0.11" }
helpful = { workspace = true }
async-openai = { version = "0.28.0" }
stub-macro = "0.1.6"
tempfile = "3.16.0"
itertools = "0.14.0"
subtype = { git = "https://github.com/DenisGorbachev/subtype" }
tokio = { workspace = true }
serde_json = { workspace = true }
clap = { workspace = true }
url = "2.5.4"
#figment = { version = "0.10.19", features = ["test", "toml"] }
xshell = { version = "0.3.0-pre.2" }
ra_ap_syntax = "0.0.277"
ra_ap_text_edit = "0.0.241"

[lints]
workspace = true


[package.metadata.cargo-machete]
ignored = ["stub-macro"]
```

### packages/oneshot_cli/Cargo.toml

```toml
[package]
name = "oneshot_cli"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]

[lints]
workspace = true
```

### packages/oneshot_common/Cargo.toml

```toml
[package]
name = "oneshot_common"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
indoc = { workspace = true }
strum = { workspace = true }
oneshot_utils = { workspace = true }
serde = { workspace = true }
fs-err = { workspace = true }
bat = { workspace = true }
itertools = { workspace = true }
quick-xml = { workspace = true }

[lints]
workspace = true
```

### packages/oneshot_rust_anthropic/Cargo.toml

```toml
[package]
name = "oneshot_rust_anthropic"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
oneshot_common = { workspace = true }
oneshot_utils = { workspace = true }
anyhow = { workspace = true }
serde = { workspace = true, optional = true }
tokio = { workspace = true }
clap = { workspace = true }
clust = { workspace = true }
clust_ext = { workspace = true }
syn = { workspace = true }
proc-macro2 = { workspace = true, features = ["span-locations"] }
fs-err = { workspace = true }
oneshot_rust = { workspace = true }
strum = { workspace = true }
prettyplease = { workspace = true }
throbber-widgets-tui = { workspace = true }
ratatui = { workspace = true }
tracing-subscriber = { workspace = true }
serde_yaml = { workspace = true }
time = { workspace = true }
save-load = { workspace = true }
constcat = { workspace = true }
directories = { workspace = true }
lazy_static = { workspace = true }
bat = { workspace = true }
quote = { workspace = true }
indexmap = { workspace = true }
rustc-hash = { workspace = true }
tempfile = { workspace = true }
lsp-types = { workspace = true }

[lints]
workspace = true

[package.metadata.cargo-machete]
ignored = ["proc-macro2"]
```

### packages/oneshot_rust/Cargo.toml

```toml
[package]
name = "oneshot_rust"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
syn = { workspace = true }
quote = { workspace = true }
serde = { workspace = true, optional = true }
oneshot_utils = { workspace = true }
oneshot_common = { workspace = true }
bat = { workspace = true }
pulldown-cmark = { workspace = true }
serde_json = { workspace = true }
duct = { workspace = true }
indexmap = { workspace = true }

[lints]
workspace = true
```

### packages/oneshot_utils/Cargo.toml

```toml
[package]
name = "oneshot_utils"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
derive-new = { workspace = true }
derive_more = { workspace = true }
fmt-derive = { workspace = true }
derive-getters = { workspace = true }
num-traits = { workspace = true }
fs-err = { workspace = true }
serde = { workspace = true, optional = true }

[lints]
workspace = true
```

### packages/pseudorust/Cargo.toml

```toml
[package]
name = "pseudorust"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]

[lints]
workspace = true
```

### packages/tasks/Cargo.toml

```toml
[package]
name = "tasks"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
serde = { workspace = true, optional = true }
derive-getters = { workspace = true }
derive-new = { workspace = true }

[lints]
workspace = true
```

### packages/clust_ext/src/lib.rs

```rust
pub mod functions;
pub mod types;
```

### packages/oneshot_aist/src/lib.rs

```rust
//! # Idea
//!
//! A CLI app that tries to "fix" a directory.
//!
//! There is a [`subtype::Validate`] trait. This function returns a `Vec<impl Error>`. If the vector is empty, then the search is complete. If the vector is not empty, then it sends a request to the LLM. The LLM returns a response that is interpreted as a list of commands applied to files in context.
//!
//! It is possible

mod traits;
mod types;

pub use traits::*;

pub use types::*;

mod functions;

pub use functions::*;

mod utils;

pub use utils::*;

mod ext;

pub use ext::*;

mod tests;
```

### packages/oneshot_aist/src/main.rs

```rust
use clap::Parser;
use oneshot_aist::{Cli, Outcome};

#[tokio::main]
async fn main() -> Outcome {
    let args = Cli::parse();
    args.run().await
}
```

### packages/oneshot_cli/src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

### packages/oneshot_common/src/lib.rs

```rust
pub mod constants;
pub mod functions;
pub mod types;
```

### packages/oneshot_rust_anthropic/src/main.rs

```rust
use clap::Parser;
use time::OffsetDateTime;

use types::cli::Cli;

pub mod functions;
pub mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_env_logger();
    init_tracing_subscriber();
    let now = OffsetDateTime::now_utc();
    Cli::parse().execute(now).await
}

fn init_env_logger() {
    // env_logger::init();
}

fn init_tracing_subscriber() {
    use tracing_subscriber::util::SubscriberInitExt;
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        // .with_max_level(tracing::Level::TRACE) // Set the maximum log level to TRACE
        .finish();
    // dbg!(&subscriber);
    subscriber.init();
}

pub mod constants;
pub mod specs;
pub mod statics;
mod tasks;
mod test_files;
```

### packages/oneshot_rust/src/lib.rs

```rust
pub mod functions;
pub mod types;
```

### packages/oneshot_utils/src/lib.rs

```rust
pub mod constants;
pub mod functions;
pub mod traits;
pub mod types;
```

### packages/pseudorust/src/lib.rs

```rust
mod specs;
```

### packages/tasks/src/main.rs

```rust
// #![feature(trace_macros)]
// #![feature(log_syntax)]

fn main() {
    println!("Hello, world!");
}

pub mod notes;
pub mod types;
```
