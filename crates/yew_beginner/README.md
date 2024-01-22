# Yew 初学者

您将需要一些工具来编译、构建、打包和调试您的 Yew 应用程序。开始时，我们建议使用 Trunk。Trunk 是 Rust 的 WASM Web 应用程序打包器。

## 安装 WebAssembly 目标

Rust 可以为不同的 “目标”（例如不同的处理器）编译源代码。基于浏览器的 WebAssembly 的编译目标称为 wasm32-unknown-unknown 。以下命令会将 WebAssembly 目标添加到您的开发环境中。

```sh
rustup target add wasm32-unknown-unknown
```

## 安装 Trunk

Trunk 是用于管理部署和打包的推荐工具，在文档和示例中都使用。

```sh
cargo install --locked trunk
```

## 参考文档

- [Yew | Home](https://yew.rs/)
- [trunkrs | Home](https://trunkrs.dev/#install)
