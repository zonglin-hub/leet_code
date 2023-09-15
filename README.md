# Leet Code

使用 Rust 解决 Leetcode Code。

先写 Docs，再写测试，以测试驱动完成 Code。

### Setup

Nushell requires a recent Rust toolchain and some dependencies; [refer to the Nu Book for up-to-date requirements](https://www.nushell.sh/book/installation.html#build-from-source). After installing dependencies, you should be able to clone+build Nu like any other Rust project:

```bash
git clone https://github.com/nushell/nushell
cd nushell
cargo build
```

### Tests

It is a good practice to cover your changes with a test. Also, try to think about corner cases and various ways how your changes could break. Cover those in the tests as well.

Tests can be found in different places:
* `/tests`

Most of the tests are built upon the `nu-test-support` crate. For testing specific features, such as running Nushell in a REPL mode, we have so called "testbins". For simple tests, you can find `run_test()` and `fail_test()` functions.


### Useful Commands

As Nushell is build using a cargo workspace consisting of multiple crates keep in mind that you may need to pass additional flags compared to how you may be used to it from a single crate project.
Read cargo's documentation for more details: https://doc.rust-lang.org/cargo/reference/workspaces.html

- Build and run Nushell:

  ```shell
  cargo run
  ```

- Build and run with dataframe support.
  ```shell
  cargo run --features=dataframe
  ```

- Run Clippy on Nushell:

  ```shell
  cargo clippy --workspace -- -D warnings -D clippy::unwrap_used
  ```
  or via the `toolkit.nu` command:
  ```shell
  use toolkit.nu clippy
  clippy
  ```

- Run all tests:

  ```shell
  cargo test --workspace
  ```

  along with dataframe tests

  ```shell
  cargo test --workspace --features=dataframe
  ```
  or via the `toolkit.nu` command:
  ```shell
  use toolkit.nu test
  test
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```
  or via the `toolkit.nu` command:
  ```shell
  use toolkit.nu fmt
  fmt --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
  or via the `toolkit.nu` command:
  ```shell
  use toolkit.nu fmt
  fmt
  ```

## 参考

- [Rust GYM](https://rustgym.com/)
- [力扣（LeetCode）官网 - 全球极客挚爱的技术成长平台](https://leetcode.cn)
