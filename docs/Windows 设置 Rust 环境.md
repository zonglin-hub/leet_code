# Windows 设置 Rust 环境

- 官网下载 [Microsoft C++ 生成工具][Microsoft C++ 生成工具]，并安装 <strong style="color: yellow;">Desktop development with C++</strong> 如果不安装无法正常编译。

- 这里我们需要一个完整的 `gcc` 运行时环境。请在 MinGW64 官网[进行下载 `x86_64-win32-seh`][MinGW64]。如不安装，编辑代码时，会影响部分函数提示。
  
  并于系统变量 `Path` 中编辑环境变量，指定 `mingw64\bin` 目录。

  ```text
  D:\program\mingw64\bin
  ```

    <details><summary><b>使用 <code>gcc -v</code> 测试 gcc 环境是否正常。</b></summary>

    ```text
    ~> gcc -v
    Using built-in specs.
    COLLECT_GCC=D:\program\mingw64\bin\gcc.exe
    COLLECT_LTO_WRAPPER=D:/program/mingw64/bin/../libexec/gcc/x86_64-w64-mingw32/8.1.0/lto-wrapper.exe
    Target: x86_64-w64-mingw32
    Configured with: ../../../src/gcc-8.1.0/configure --host=x86_64-w64-mingw32 --build=x86_64-w64-mingw32 --target=x86_64-w64-mingw32 --prefix=/mingw64 --with-sysroot=/c/mingw810/x86_64-810-win32-seh-rt_v6-rev0/mingw64 --enable-shared --enable-static --disable-multilib --enable-languages=c,c++,fortran,lto --enable-libstdcxx-time=yes --enable-threads=win32 --enable-libgomp --enable-libatomic --enable-lto --enable-graphite --enable-checking=release --enable-fully-dynamic-string --enable-version-specific-runtime-libs --disable-libstdcxx-pch --disable-libstdcxx-debug --enable-bootstrap --disable-rpath --disable-win32-registry --disable-nls --disable-werror --disable-symvers --with-gnu-as --with-gnu-ld --with-arch=nocona --with-tune=core2 --with-libiconv --with-system-zlib --with-gmp=/c/mingw810/prerequisites/x86_64-w64-mingw32-static --with-mpfr=/c/mingw810/prerequisites/x86_64-w64-mingw32-static --with-mpc=/c/mingw810/prerequisites/x86_64-w64-mingw32-static --with-isl=/c/mingw810/prerequisites/x86_64-w64-mingw32-static --with-pkgversion='x86_64-win32-seh-rev0, Built by MinGW-W64 project' --with-bugurl=https://sourceforge.net/projects/mingw-w64 CFLAGS='-O2 -pipe -fno-ident -I/c/mingw810/x86_64-810-win32-seh-rt_v6-rev0/mingw64/opt/include -I/c/mingw810/prerequisites/x86_64-zlib-static/include -I/c/mingw810/prerequisites/x86_64-w64-mingw32-static/include' CXXFLAGS='-O2 -pipe -fno-ident -I/c/mingw810/x86_64-810-win32-seh-rt_v6-rev0/mingw64/opt/include -I/c/mingw810/prerequisites/x86_64-zlib-static/include -I/c/mingw810/prerequisites/x86_64-w64-mingw32-static/include' CPPFLAGS=' -I/c/mingw810/x86_64-810-win32-seh-rt_v6-rev0/mingw64/opt/include -I/c/mingw810/prerequisites/x86_64-zlib-static/include -I/c/mingw810/prerequisites/x86_64-w64-mingw32-static/include' LDFLAGS='-pipe -fno-ident -L/c/mingw810/x86_64-810-win32-seh-rt_v6-rev0/mingw64/opt/lib -L/c/mingw810/prerequisites/x86_64-zlib-static/lib -L/c/mingw810/prerequisites/x86_64-w64-mingw32-static/lib '
    Thread model: win32
    gcc version 8.1.0 (x86_64-win32-seh-rev0, Built by MinGW-W64 project)
    ```

    </details>

- 官网下载 `rustup-init.exe` 并安装，[点击这里下载！][download_rustup]

## Windows 安装 Rust 安装太慢解决办法

打开 `powershell` 分别执行下面两行代码：

作用：国内加速通道，注：这里时临时变量

```powershell
$ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'
$ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'
```

## 配置 cargo 国内源

找到当前用户目录下 `.cargo` 文件夹，建立 `config` 文件：

```bash
touch ~/.cargo/config
vim ~/.cargo/config
```

并输入下面内容：

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```

删除 `.package-cache`

```bash
~/.cargo> rm -rf registry
~/.cargo> rm -rf .package-cache
```

## Rust 更新

### 升级 Rust 「stable _稳定版_」

<details><summary><b>可以执行 <code>rustup update</code> 来升级 Rust 「Stable Release <i>稳定版本</i>」</b></summary>

```bash
~> rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2023-12-28, rust version 1.75.0 (82e1608df 2023-12-21)
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
 16.9 MiB /  16.9 MiB (100 %)   6.2 MiB/s in  2s ETA:  0s
info: downloading component 'rust-src'
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 14.3 MiB /  14.3 MiB (100 %)   4.8 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 17.9 MiB /  17.9 MiB (100 %)   7.7 MiB/s in  2s ETA:  0s
info: downloading component 'rustc'
 58.7 MiB /  58.7 MiB (100 %)   7.3 MiB/s in  8s ETA:  0s
info: downloading component 'rustfmt'
info: removing previous version of component 'rust-std' for 'wasm32-unknown-unknown'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
 16.9 MiB /  16.9 MiB (100 %)  13.9 MiB/s in  3s ETA:  0s
info: installing component 'rust-src'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.3 MiB /  14.3 MiB (100 %)   2.0 MiB/s in  4s ETA:  0s
info: installing component 'rust-std'
 17.9 MiB /  17.9 MiB (100 %)  13.7 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 58.7 MiB /  58.7 MiB (100 %)  15.2 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: checking for self-update

  stable-x86_64-pc-windows-msvc updated - rustc 1.75.0 (82e1608df 2023-12-21) (from rustc 1.74.1 (a28077b28 2023-12-04))

info: cleaning up downloads & tmp directories
```

</details>

### 安装 Rust 「Nightly Build _夜间构建_」

<details><summary><b>可以执行 <code>rustup install nightly</code> 来安装 「Nightly Build <i>夜间构建</i>」</b></summary>

```bash
~> rustup install nightly
info: syncing channel updates for 'nightly'
info: downloading toolchain manifest
info: downloading component 'rustc'
info: downloading component 'rust-std'
info: downloading component 'rust-docs'
info: downloading component 'cargo'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'rust-docs'
info: installing component 'cargo'

nightly installed: rustc 1.9.0-nightly (02310fd31 2016-03-19)
```

</details>

### 切换默认版本

选择稳定版或者 `nightly` 版，如果想长期使用 `nightly` 版。

```bash
rustup default nightly
```

<p>&nbsp;</p>

卸载 Rust

在任何时候如果您想卸载 Rust，您可以运行 `rustup self uninstall`。

卸载 Rust nightly 可以使用 `rustup toolchain uninstall nightly-x86_64-pc-windows-msvc`。

查询 rustup 安装的工具链 请使用 `rustup show` 或 `rustup toolchain list`

<p>&nbsp;</p>

## 异常

<strong>Rust 版本更新错误记录：Os { code: 5, kind: PermissionDenied ...}</strong>

<pre>
PS C:\Users\liuzonglin> rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: syncing channel updates for 'nightly-x86_64-pc-windows-msvc'
info: checking for self-update

   stable-x86_64-pc-windows-msvc unchanged - rustc 1.74.1 (a28077b28 2023-12-04)
  nightly-x86_64-pc-windows-msvc unchanged - rustc 1.77.0-nightly (2df6406b8 2023-12-26)

info: cleaning up downloads & tmp directories
thread 'main' panicked at 'Unable to clean up C:\Users\liuzonglin\.rustup\tmp: Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }', src\utils\utils.rs:650:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
</pre>

<strong>原因：</strong>

<pre>
tmp 无法删除，目前没有正在运行的 rust 项目，应当是有其它 rust 的进程还在活动，检查 vscode, rust-analyzer 还在活动，停止插件或关闭 vscode。
</pre>

<p>&nbsp;</p>

<strong>如果在运行 cargo 的时候：Blocking waiting for file lock on package cache</strong>

请产生 `.cargo` 文件夹下面的 `.package_cache` 文件：

```sh
rm ~/.cargo/.package-cache
```

<strong>原因：</strong>

<pre>
cargo 堵塞问题。
</pre>

<p>&nbsp;</p>

## 参考文档

- [Substrate 开发系列 - 环境搭建][substrate_dev_install]
- [清华大学开源软件镜像站 Rust crates.io 索引][tsinghua_university]
- [RsProxy国内镜像代理][RsProxy国内镜像代理]
- [安装 Rust - Rust 程序设计语言 (rust-lang.org)][download_rustup]
- [Windows安装Rust指南 - 哔哩哔哩 (bilibili.com)][Windows安装Rust指南]

[Microsoft C++ 生成工具]: https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/
[download_rustup]: https://www.rust-lang.org/zh-CN/tools/install
[MinGW64]: https://sourceforge.net/projects/mingw-w64/files/
[substrate_dev_install]: https://learnblockchain.cn/article/1069
[tsinghua_university]: https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index.git/
[RsProxy国内镜像代理]: https://rsproxy.cn/
[Windows安装Rust指南]: https://www.bilibili.com/read/cv17841257
