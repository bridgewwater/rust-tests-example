[![ci](https://github.com/bridgewwater/rust-tests-example/actions/workflows/ci.yml/badge.svg)](https://github.com/bridgewwater/rust-tests-example/actions/workflows/ci.yml)

[![doc.rs](https://img.shields.io/docsrs/rust-tests-example)](https://docs.rs/rust-tests-example)
[![crates.io version](https://img.shields.io/crates/v/rust-tests-example)](https://crates.io/crates/rust-tests-example)
[![crates.io download latest](https://img.shields.io/crates/dv/rust-tests-example.svg)](https://crates.io/crates/rust-tests-example)
[![crates.io license](https://img.shields.io/crates/l/rust-tests-example)](https://crates.io/crates/rust-tests-example)
[![deps.rs dependency](https://deps.rs/crate/rust-tests-example/latest/status.svg)](https://deps.rs/repo/github/rust-tests-example/latest)

[![GitHub license](https://img.shields.io/github/license/bridgewwater/rust-tests-example)](https://github.com/bridgewwater/rust-tests-example)
[![GitHub latest SemVer tag)](https://img.shields.io/github/v/tag/bridgewwater/rust-tests-example)](https://github.com/bridgewwater/rust-tests-example/tags)
[![GitHub release)](https://img.shields.io/github/v/release/bridgewwater/rust-tests-example)](https://github.com/bridgewwater/rust-tests-example/releases)

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](.github/CONTRIBUTING_DOC/CODE_OF_CONDUCT.md)
[![GitHub contributors](https://img.shields.io/github/contributors/bridgewwater/rust-tests-example)](https://github.com/bridgewwater/rust-tests-example/graphs/contributors)

We welcome community contributions to this project.

Please read [Contributor Guide](.github/CONTRIBUTING_DOC/CONTRIBUTING.md) for more information on how to get started.

请阅读有关 [贡献者指南](.github/CONTRIBUTING_DOC/zh-CN/CONTRIBUTING.md) 以获取更多如何入门的信息

## env

- rust version `1.82.0`

## dependencies

```bash
# use testdir
$ cargo add --dev testdir

# Uses the quickcheck crate for property-based tests
$ cargo add --dev quickcheck quickcheck_macros

# using assert_cmd to test the command line interface of our main binary
$ cargo add --dev assert_cmd
# benchmark
$ cargo add --dev criterion
```

## usage

### show test target list

```bash
$ cargo test -q -- --list
```

### run test

```bash
$ cargo test --lib -- --show-output
```

more case see [Makefile](Makefile)

# rust package and moudle

## package

- define at `Cargo.toml`

| cmd | use as define |
| --- | --- |
| cargo build | `[profile.dev]` |
| cargo build --release | `[profile.release]` |
| cargo test | `[profile.test]` |

- if want use release and open debug symbol must setting as

```toml
[profile.release]
debug = true
```

### most use of attribute

| `#[cfg(...)]`  | when to enable |
|----------------|-----------------|
| test           | as `cargo test` or `cargo --test` |
| feture = "robots"        | as user define `robots`, this will in `Cargo.toml` config at `[features]` |
| not(A) | with a configuration predicate. It is true if its predicate is false and false if its predicate is true |
| all(A, B) | just like `&&` with a comma separated list of configuration predicates. It is false if at least one predicate is false. If there are no predicates, it is true. |
| any(A, B) | just like `||` with a comma separated list of configuration predicates. It is true if at least one predicate is true. If there are no predicates, it is false. |
| debug_assertions | as start debug assertion for Non-optimized builds by [debug_assertions](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions) |
| target_os = "macos" | as build in macos [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os) |
| target_arch = "x86_64" | as build for x86_64 [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch) |
| target_pointer_width = "64" | as build for 64bit [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width) |

### inline attribute

[https://doc.rust-lang.org/reference/attributes/codegen.html?highlight=%23%5Binline%5D#the-inline-attribute](https://doc.rust-lang.org/reference/attributes/codegen.html?highlight=%23%5Binline%5D#the-inline-attribute)

# test

```bash
$ cargo test
```

## test compile

- use `cargo build` or `cargo build --release` will pass `#[test]` code
- most tests case in mod for management and unit-testing not be error as

```log
warning: function is never used: `foo`
```

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

this way let module only for test

if want only run one test case just like

```bash
RUST_TEST_THREADS=1 cargo test
```

## will panic

```rs
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_div_by_zero() {
        let _ = divide(1, 0);
    }
```
[should_panic](https://doc.rust-lang.org/reference/attributes/testing.html?highlight=should_panic#the-should_panic-attribute)

A function annotated with the `test` attribute that returns () can also be annotated with the `should_panic` attribute.
The `should_panic` attribute makes the test only pass if it actually panics.


# doc

## create doc

```bash
$ cargo doc --no-deps
# will create html page in `target/doc/${name}`
$ cargo doc --no-deps --open
# create and open
```

rust document format as `Markdown`

- begin as `///` will some as `#[doc]`
- begin as `//!` will some as `#![doc]`
- code example

```rs
/// # Example
///
/// ```rs
/// if foo.works(){
/// 	println!("foo works");
/// }
/// ```
```

### doc attr


#### crate

- html_favicon_url

```rs
#![doc(html_favicon_url = "https://example.com/favicon.ico")]
```

- html_logo_url

```rs
#![doc(html_logo_url = "https://example.com/logo.jpg")]
```

- html_playground_url

```rs
#![doc(html_playground_url = "https://playground.example.com/")]
```

- issue_tracker_base_url

```rs
#![doc(issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/")]
```

- html_root_url

create out crate item link

```rs
#![doc(html_root_url = "https://docs.rs/serde/1.0")]
```

- html_no_source

```rs
#![doc(html_no_source)]
```

#### item

- inline  Rust 2018+, if pub use your depend，rustdoc can not inline modules, except add

```rs
#[doc(no_inline)]
pub use bar::Bar;
```

- hidden not in doc out

```rs
#[doc(hidden)]
```

- alias most use in FFI, like:

```rs
pub struct Obj {
    inner: *mut ffi::Obj,
}

impl Obj {

    #[doc(alias = "lib_name_do_something")]
    pub fn do_something(&mut self) -> i32 {
        unsafe { ffi::lib_name_do_something(self.inner) }
    }
}
```

user can search `lib_name_do_something` to show method as `do_something`

## create doc for test

in doc example code will add tests, can show as

```bash
$ cargo test -- --list
```

- if want code not run test can add `no_run` like this

```rs
/// # Example
///
/// ```no_run
/// use test_and_doc::divide;
///
/// assert_eq!(1, divide(1, 1));
/// assert_eq!(3, divide(9, 3));
/// ```
```

- if not want doc compile, just add `ignore` like this

```rs
/// # Example
///
/// ```ignore
/// use test_and_doc::divide;
///
/// assert_eq!(1, divide(1, 1));
/// assert_eq!(3, divide(9, 3));
/// ```
```

- if is other thing just use `text` or `c++`

```rs
/// # Example
///
/// ```c++
/// use test_and_doc::divide;
///
/// assert_eq!(1, divide(1, 1));
/// assert_eq!(3, divide(9, 3));
/// ```