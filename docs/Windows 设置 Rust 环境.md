# Windows 设置 Rust 环境

## 安装 Rust

- 官网下载 [Microsoft C++ 生成工具][Microsoft C++ 生成工具] 不安装无法编译

    <details><summary><b>参考</b></summary>

    ![image](https://img2023.cnblogs.com/blog/2402369/202309/2402369-20230923114520833-509672411.png)
    ![image](https://img2023.cnblogs.com/blog/2402369/202309/2402369-20230923114559005-2087802634.png)

    </details>

- 官网下载 `rustup-init.exe` 并安装，下载地址[在这！][download_rustup]

- 官网下载 MinGW64 [x86_64-win32-seh][MinGW64] 不安装代码提示，不全

    <details><summary><b>参考</b></summary>

    ![image](https://img2023.cnblogs.com/blog/2402369/202309/2402369-20230923114614643-572766821.png)
    ![image](https://img2023.cnblogs.com/blog/2402369/202309/2402369-20230923114635026-1714409751.png)

    ```powershell
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

## windows 安装 Rust 安装太慢解决办法

打开 `powershell` 分别执行下面两行代码：

```powershell
$ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'
$ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'
```

## 配置 cargo 国内源

找到当前用户目录下 .cargo文件夹，建立config文件：

```bash
touch ~/.cargo/config
vim ~/.cargo/config
```

输入下面内容：

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

```sh
~/.cargo> rm -rf registry
~/.cargo> rm -rf .package-cache
```

## 解决 cargo 堵塞 blocking 问题

如果在运行 cargo 的时候，出现：<u>Blocking waiting for file lock on package cache</u>

请产生 `.cargo` 文件夹下面的 `.package_cache` 文件：

```sh
rm ~/.cargo/.package-cache
```

## Rust 更新

稳定版和nightly版的升级

```rust
~> rustup update                                                                                                  2023/07/16 01:06:05 下午
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: checking for self-update

  stable-x86_64-pc-windows-msvc unchanged - rustc 1.71.0 (8ede3aae2 2023-07-12)

info: cleaning up downloads & tmp directories
```

rustup升级

```sh
~> rustup self update                                                                                             2023/07/16 01:11:12 下午
info: checking for self-update
  rustup unchanged - 1.26.0
```

nightly版安装

```sh
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

查询版本

```sh
rustup run nightly rustc --version
```

选择稳定版或者nightly版

如果想长期使用 nightly版。

```rust
rustup default nightly
```

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
