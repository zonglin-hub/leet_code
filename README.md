# Leet Code

使用 Rust 解决 Leetcode Code。

先写 Docs，再写测试，以测试驱动完成 Code。


### Setup

Nushell 需要最近的 Rust 工具链和一些依赖项； [有关最新要求，请参阅 Nu Book](https://www.nushell.sh/book/installation.html#build-from-source). 安装依赖项后，您应该能够像任何其他 Rust 项目一样 clone + build Nu：

```bash
git clone https://github.com/nushell/nushell
cd nushell
cargo build
```

### Tests

用测试覆盖您的更改是一个很好的做法。此外，尝试考虑角落情况和各种方法如何破坏您的更改。也涵盖测试中的那些。

测试可以在不同的地方找到：
* `/tests`


### Useful Commands

由于 Nushell 是使用由多个板条箱组成的货物工作区构建的，请记住，与从单个板条箱项目中使用它的方式相比，您可能需要传递额外的标志。
阅读 cargo 的文档档 了解 更多详情: https://doc.rust-lang.org/cargo/reference/workspaces.html

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
