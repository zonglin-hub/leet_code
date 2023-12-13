### 使用 cargo expand 查看被宏隐藏的代码

使用 VScode 安装扩展 `Rust Macro Expand` 

需要安装以下软件:

- [cargo-expand](https://github.com/dtolnay/cargo-expand) A cargo crate for easier handling of compiler commands
- Rust nightly compiler, you can install it with `rustup toolchain install nightly`

#### cargo expand简介

- `cargo expand` 目前这个需要安装 `nightly` 的 `toolchain`，`rustup toolchain install nightly-x86_64-unknown-linux-gnu`

    ```sh
    ~> rustup toolchain install nightly
    info: syncing channel updates for 'nightly-x86_64-pc-windows-msvc'
    info: latest update on 2023-07-26, rust version 1.73.0-nightly (864bdf784 2023-07-25)
    info: downloading component 'cargo'
    info: downloading component 'clippy'
    info: downloading component 'rust-docs'
    info: downloading component 'rust-std'
    info: downloading component 'rustc'
    58.2 MiB /  58.2 MiB (100 %)  40.5 MiB/s in  1s ETA:  0s
    info: downloading component 'rustfmt'
    info: installing component 'cargo'
    info: installing component 'clippy'
    info: installing component 'rust-docs'
    13.7 MiB /  13.7 MiB (100 %)   2.0 MiB/s in  4s ETA:  0s
    info: installing component 'rust-std'
    23.6 MiB /  23.6 MiB (100 %)  16.2 MiB/s in  1s ETA:  0s
    info: installing component 'rustc'
    58.2 MiB /  58.2 MiB (100 %)  17.4 MiB/s in  3s ETA:  0s
    info: installing component 'rustfmt'

    nightly-x86_64-pc-windows-msvc installed - rustc 1.73.0-nightly (864bdf784 2023-07-25)

    info: checking for self-update
    ```

- 安装命令：`cargo +nightly install cargo-expand`

    ```sh
    ~> cargo +nightly install cargo-expand
        Updating `ustc` index
    Downloaded cargo-expand v1.0.62 (registry `ustc`)
    Downloaded 1 crate (26.8 KB) in 1.29s
    Installing cargo-expand v1.0.62
    Downloaded ansi_colours v1.2.2 (registry `ustc`)
    ...
        Finished release [optimized] target(s) in 1m 25s
    Installing C:\Users\liuzonglin\.cargo\bin\cargo-expand.exe
    Installed package `cargo-expand v1.0.62` (executable `cargo-expand.exe`)
    ```

#### 使用方法

- 源代码

    ```rust
    #![allow(non_camel_case_types)]
    use async_trait::async_trait;
    fn main() {}
    #[async_trait]
    trait example {
        async fn fetch(trace_id: &str, span_id: &str);
    }
    ```

- 运行 `cargo expand`

    ```sh
    ~> cargo expand
        Checking lifetime_kata v0.1.0 (D:\.github\rust\lifetime_kata)
        Finished dev [unoptimized + debuginfo] target(s) in 0.04s

    #![feature(prelude_import)]
    #![allow(non_camel_case_types)]
    #[prelude_import]
    use std::prelude::rust_2021::*;
    #[macro_use]
    extern crate std;
    use async_trait::async_trait;
    fn main() {}
    trait example {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn fetch<'life0, 'life1, 'async_trait>(
            trace_id: &'life0 str,
            span_id: &'life1 str,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait;
    }
    ```
