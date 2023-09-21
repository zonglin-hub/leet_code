# Leet Code

使用 Rust 解决 Leetcode Code。

先写 Docs，再写测试，以测试驱动完成 Code。


### Setup

这里使用 `Nushell` 工具简化命令；[请参阅 Nu Book](https://www.nushell.sh/book/installation.html#build-from-source)。


### Tests

用测试覆盖您的更改是一个很好的做法。此外，尝试考虑角落情况和各种方法如何破坏您的更改。也涵盖测试中的那些。

测试可以在不同的地方找到：
* `/tests`


### Useful Commands

由于 `Nushell` 是使用由多个板条箱组成的货物工作区构建的，请记住，与从单个板条箱项目中使用它的方式相比，您可能需要传递额外的标志。

阅读 `cargo` 相关文档了解更多详情: https://doc.rust-lang.org/cargo/reference/workspaces.html

- 使用 `Nushell` 运行你的 `Rust` 项目:

  ```shell
  use toolkit.nu clippy
  clippy
  ```

- 运行所有测试

  ```shell
  use toolkit.nu test
  test
  ```

- 检查是否存在代码格式问题

  ```shell
  use toolkit.nu fmt
  fmt --check
  ```

- 格式化项目中的代码

  ```shell
  use toolkit.nu fmt
  fmt
  ```


## 参考

- [Rust GYM](https://rustgym.com/)
- [力扣（LeetCode）官网 - 全球极客挚爱的技术成长平台](https://leetcode.cn)
