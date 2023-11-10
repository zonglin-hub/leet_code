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

```
rm ~/.cargo/.package-cache
```

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
