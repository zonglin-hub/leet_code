# rustup -h

<pre>
$ rustup -h
rustup 1.26.0 (5af9b9484 2023-04-05)
The Rust toolchain installer

使用方法：
    $ rustup [选项] [+toolchain] <+子命令>

参数：
    <+toolchain>    发布渠道（例如 +stable）或自定义工具链以设置覆盖

选项：
    -v, --verbose    启用详细输出
    -q, --quiet      禁用进度输出
    -h, --help       打印帮助信息
    -V, --version    打印版本信息

子命令：
    show           显示活动和解压的工具链或配置文件
    update         更新 Rust 工具链和 rustup
    check          检查 Rust 工具链和 rustup 的更新
    default        设置默认工具链
    toolchain      修改或查询已安装的工具链
    target         修改工具链支持的目标
    component      修改工具链安装的组件
    override       修改目录工具链覆盖
    run            使用为给定工具链配置的环境运行命令
    which          显示将为给定命令运行的二进制文件
    doc            打开当前工具链的文档
    self           修改 rustup 安装
    set            修改 rustup 设置
    completions    为您的 shell 生成制表完成脚本
    help           打印此消息或给定子命令的帮助信息

说明：
    Rustup 是从 Rust 程序设计语言官方发布渠道安装 ，使您可以轻松地在稳定版、测试版和夜间版编译器之间切换并保持更新。
    它通过为常见平台的标准库提供二进制构建，使交叉编译变得简单。

    如果您是 Rust 的新手，请考虑运行 `rustup doc --book` 来学习 Rust。

示例:
    升级 Rust 「stable _稳定版_」：

        $ rustup update

    显示工具链信息：

        $ rustup show

</pre>

<p>&nbsp;</p>

## rustup show -h

<pre>
~> rustup show -h
rustup-show
显示活动工具链和安装的工具链或配置文件

使用方法：
    rustup show [选项] [子命令]

选项：
    -v, --verbose    启用带有 rustc 信息的详细输出，用于所有安装的工具链
    -h, --help       打印帮助信息

子命令：
    active-toolchain    显示活动工具链
    home                显示计算得出的 RUSTUP_HOME 值
    profile             显示当前配置文件
    help                打印此消息或给定子命令的帮助信息

说明：
    显示活动工具链的名称和 rustc 的版本。

    如果活动工具链安装了支持其他编译目标的支持，那么它们也会被列出。

    如果安装了多个工具链，那么所有安装的工具链也会被列出。

示例:
    显示计算得出的 RUSTUP_HOME 值：

        $ rustup show home

</pre>

### rustup show active-toolchain -h

<pre>
~> rustup show active-toolchain -h
rustup-show-active-toolchain
Show the active toolchain

USAGE:
    rustup show active-toolchain [OPTIONS]

OPTIONS:
    -v, --verbose    Enable verbose output with rustc information
    -h, --help       Print help information

DISCUSSION:
    Shows the name of the active toolchain.

    This is useful for figuring out the active tool chain from
    scripts.

    You should use `rustc --print sysroot` to get the sysroot, or
    `rustc --version` to get the toolchain version.
</pre>

### rustup show home -h

<pre>
~> rustup show home -h
rustup-show-home
显示 RUSTUP_HOME 的计算值

使用方法：
    rustup show home

选项：
    -h, --help    打印帮助信息
</pre>

### rustup show profile -h

<pre>
~> rustup show profile -h
rustup-show-profile
显示当前配置文件

使用方法：
    rustup show profile

选项：
    -h, --help    打印帮助信息
</pre>

<p>&nbsp;</p>

## rustup update -h

<pre>
~> rustup update -h
rustup-update
更新 Rust 工具链和 rustup

使用方法：
    rustup update [OPTIONS] [toolchain]...

参数：
    <toolchain>...    工具链名称，例如 'stable'、'nightly' 或 '1.8.0'。更多信息请参见 `rustup help toolchain`

选项：
        --no-self-update    在运行 `rustup update` 命令时不执行自我更新
        --force             即使某些组件缺失，也强制执行更新
        --force-non-host    安装需要模拟器的工具链。请参阅
                            https://github.com/rust-lang/rustup/wiki/Non-host-toolchains
    -h, --help              打印帮助信息

讨论：
    如果没有指定工具链，`update` 命令会从官方发布渠道更新每个已安装的工具链，然后更新 rustup 本身。

    如果提供了工具链参数，则 `update` 会更新该工具链，这与 `rustup toolchain install` 相同。
</pre>

## rustup check -h

<pre>
~> rustup check -h
rustup-check
检查 Rust 工具链和 rustup 的更新

使用方法：
    rustup check

选项：
    -h, --help    打印帮助信息
</pre>

## rustup default -h

<pre>
~> rustup default -h
rustup-default
设置默认工具链

使用方法：
    rustup default [工具链]

参数：
    <工具链>    工具链名称，例如 'stable'，'nightly' 或 '1.8.0'。更多信息请参见 `rustup help toolchain`

选项：
    -h, --help    打印帮助信息

讨论：
    将默认工具链设置为指定的工具链。如果工具链
    尚未安装，则首先安装它。
</pre>

<p>&nbsp;</p>

## rustup toolchain -h

<pre>
~> rustup toolchain -h
rustup-toolchain
修改或查询已安装的工具链

使用方法：
    rustup toolchain <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    list         列出已安装的工具链
    install      安装或更新指定的工具链
    uninstall    卸载工具链
    link         通过符号链接到目录创建自定义工具链
    help         打印此消息或给定子命令的帮助

讨论：
    许多 `rustup` 命令都涉及 *工具链*，即 Rust 编译器的单个安装。`rustup` 支持多种类型的工具链。
    最基本的工具链跟踪官方发布频道：'stable'，'beta' 和 'nightly'；
    但 `rustup` 也可以从官方存档安装工具链，用于替代主机平台，以及从本地构建。

    标准发布频道工具链名称的形式如下：
        <频道>-<日期>-<主机>

        <频道>       = stable|beta|nightly|<major.minor>|<major.minor.patch>
        <日期>       = YYYY-MM-DD
        <主机>       = target-triple

    '频道' 是一个命名发布频道，一个主要和次要版本号，如 `1.42`，或者一个完全指定的版本号，如 `1.42.0`。
    频道名称可以可选地附加一个存档日期，如 `nightly-2014-12-18`，在这种情况下，工具链将从该日期的存档中下载。

    主机可以指定为目标三元组。这对于在 64 位平台上安装 32 位编译器
    或在 Windows 上安装 [MSVC-based 工具链] 最有用。例如：

        $ rustup toolchain install stable-x86_64-pc-windows-msvc

    为了方便，省略的目标三元组元素将被推断，所以上述命令可以写成：

        $ rustup toolchain install stable-msvc

    `rustup default` 命令可以用于在单个命令中安装并设置所需工具链为默认：

        $ rustup default stable-msvc

    `rustup` 还可以管理符号链接的本地工具链构建，这通常用于开发 Rust 本身。
    更多信息请参见 `rustup toolchain help link`。
</pre>

### rustup toolchain list -h

<pre>
~> rustup toolchain list -h
rustup-toolchain-list
List installed toolchains

USAGE:
    rustup toolchain list [OPTIONS]

OPTIONS:
    -v, --verbose    Enable verbose output with toolchain information
    -h, --help       Print help information
</pre>

### rustup toolchain install -h

<pre>
~> rustup toolchain install -h
rustup-toolchain-install
Install or update a given toolchain

USAGE:
    rustup toolchain install [OPTIONS] <toolchain>...

ARGS:
    <toolchain>...    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see `rustup help
                      toolchain`

OPTIONS:
        --profile <profile>            [possible values: minimal, default, complete]
    -c, --component <components>...    Add specific components on installation
    -t, --target <targets>...          Add specific targets on installation
        --no-self-update               Don't perform self update when running the`rustup toolchain install` command
        --force                        Force an update, even if some components are missing
        --allow-downgrade              Allow rustup to downgrade the toolchain to satisfy your component choice
        --force-non-host               Install toolchains that require an emulator. See
                                       https://github.com/rust-lang/rustup/wiki/Non-host-toolchains
    -h, --help                         Print help information
</pre>

### rustup toolchain uninstall -h

<pre>
~> rustup toolchain uninstall -h
rustup-toolchain-uninstall
Uninstall a toolchain

USAGE:
    rustup toolchain uninstall <toolchain>...

ARGS:
    <toolchain>...    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see `rustup help
                      toolchain`

OPTIONS:
    -h, --help    Print help information
</pre>

### rustup toolchain link -h

<pre>
~> rustup toolchain link -h
rustup-toolchain-link
Create a custom toolchain by symlinking to a directory

USAGE:
    rustup toolchain link <toolchain> <path>

ARGS:
    <toolchain>    Custom toolchain name
    <path>         Path to the directory

OPTIONS:
    -h, --help    Print help information

DISCUSSION:
    'toolchain' is the custom name to be assigned to the new toolchain.
    Any name is permitted as long as it does not fully match an initial
    substring of a standard release channel. For example, you can use
    the names 'latest' or '2017-04-01' but you cannot use 'stable' or
    'beta-i686' or 'nightly-x86_64-unknown-linux-gnu'.

    'path' specifies the directory where the binaries and libraries for
    the custom toolchain can be found. For example, when used for
    development of Rust itself, toolchains can be linked directly out of
    the build directory. After building, you can test out different
    compiler versions as follows:

        $ rustup toolchain link latest-stage1 build/x86_64-unknown-linux-gnu/stage1
        $ rustup override set latest-stage1

    If you now compile a crate in the current directory, the custom
    toolchain 'latest-stage1' will be used.
</pre>

<p>&nbsp;</p>

## rustup target -h

<pre>
~> rustup target -h
rustup-target
修改工具链的支持目标

使用方法：
    rustup target <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    list      列出已安装和可用的目标
    add       将目标添加到 Rust 工具链中
    remove    从 Rust 工具链中移除目标
    help      打印此消息或给定子命令的帮助
</pre>

### rustup target list -h

<pre>
~> rustup target list -h
rustup-target-list
List installed and available targets

USAGE:
    rustup target list [OPTIONS]

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
        --installed                List only installed targets
    -h, --help                     Print help information
</pre>

### rustup target add -h

<pre>
~> rustup target add -h
rustup-target-add
Add a target to a Rust toolchain

USAGE:
    rustup target add [OPTIONS] <target>...

ARGS:
    <target>...    List of targets to install; "all" installs all available targets

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
    -h, --help                     Print help information
</pre>

### rustup target remove -h

<pre>
~> rustup target remove -h
rustup-target-remove
Remove a target from a Rust toolchain

USAGE:
    rustup target remove [OPTIONS] <target>...

ARGS:
    <target>...    List of targets to uninstall

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
    -h, --help                     Print help information
</pre>

<p>&nbsp;</p>

## rustup component -h

<pre>
~> rustup component -h
rustup-component
修改工具链的已安装组件

使用方法：
    rustup component <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    list      列出已安装和可用的组件
    add       将组件添加到 Rust 工具链中
    remove    从 Rust 工具链中移除组件
    help      打印此消息或给定子命令的帮助
</pre>

### rustup component list -h

<pre>
~> rustup component list -h
rustup-component-list
List installed and available components

USAGE:
    rustup component list [OPTIONS]

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
        --installed                List only installed components
    -h, --help                     Print help information
</pre>

### rustup component add -h

<pre>
~> rustup component add -h
rustup-component-add
Add a component to a Rust toolchain

USAGE:
    rustup component add [OPTIONS] <component>...

ARGS:
    <component>...

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
        --target <target>
    -h, --help                     Print help information
</pre>

### rustup component remove -h

<pre>
~> rustup component remove -h
rustup-component-remove
Remove a component from a Rust toolchain

USAGE:
    rustup component remove [OPTIONS] <component>...

ARGS:
    <component>...

OPTIONS:
        --toolchain <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see
                                   `rustup help toolchain`
        --target <target>
    -h, --help                     Print help information
</pre>

<p>&nbsp;</p>

## rustup override -h

<pre>
~> rustup override -h
rustup-override
修改目录工具链覆盖

使用方法：
    rustup override <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    list     列出目录工具链覆盖
    set      为目录设置覆盖工具链
    unset    移除目录的覆盖工具链
    help     打印此消息或给定子命令的帮助

讨论：
    覆盖配置 Rustup 在运行在特定目录时使用特定的工具链。

    目录可以使用 `rustup override` 被分配自己的 Rust 工具链。
    当一个目录有覆盖时，任何在目录内部或其子目录中运行 `rustc` 或 `cargo` 的时候，都会调用覆盖工具链。

    要固定到特定的夜间构建：

        $ rustup override set nightly-2014-12-18

    或者特定的稳定版本：

        $ rustup override set 1.0.0

    要查看活动工具链，请使用 `rustup show`。
    要移除覆盖并再次使用默认工具链，请使用 `rustup override unset`。
</pre>

### rustup override list -h

<pre>
~> rustup override list -h
rustup-override-list
List directory toolchain overrides

USAGE:
    rustup override list

OPTIONS:
    -h, --help    Print help information
</pre>

### rustup override set -h

<pre>
~> rustup override set -h
rustup-override-set
Set the override toolchain for a directory

USAGE:
    rustup override set [OPTIONS] <toolchain>

ARGS:
    <toolchain>    Toolchain name, such as 'stable', 'nightly', or '1.8.0'. For more information see `rustup help
                   toolchain`

OPTIONS:
        --path <path>    Path to the directory
    -h, --help           Print help information
</pre>

### rustup override unset -h

<pre>
~> rustup override unset -h
rustup-override-unset
Remove the override toolchain for a directory

USAGE:
    rustup override unset [OPTIONS]

OPTIONS:
        --path <path>    Path to the directory
        --nonexistent    Remove override toolchain for all nonexistent directories
    -h, --help           Print help information

DISCUSSION:
    If `--path` argument is present, removes the override toolchain
    for the specified directory. If `--nonexistent` argument is
    present, removes the override toolchain for all nonexistent
    directories. Otherwise, removes the override toolchain for the
    current directory.
</pre>

<p>&nbsp;</p>

## rustup run -h

<pre>
~> rustup run -h
rustup-run
使用为给定工具链配置的环境运行命令

使用方法：
    rustup run [选项] <工具链> <命令>...

参数：
    <工具链>     工具链名称，例如 'stable'、'nightly' 或 '1.8.0'。更多信息请参见 `rustup help toolchain`
    <命令>...

选项：
        --install    如果需要，安装请求的工具链
    -h, --help       打印帮助信息

讨论：
    配置环境以使用给定的工具链，然后运行指定的程序。
    命令可以是任何程序，不仅仅是 `rustc` 或 `cargo`。
    这可以用于测试任意的工具链，而不需要设置覆盖。

    `rustup` 明确代理的命令（如 `rustc` 和 `cargo`）也有一个简写形式可用。可以通过使用 `+工具链`
    作为第一个参数来设置工具链。这些是等价的：

        $ cargo +nightly build

        $ rustup run nightly cargo build
</details>
<details><summary style="line-height: 0"><p>&nbsp;&nbsp;&nbsp;</p></summary>

</pre>

## rustup which -h

<pre>
~> rustup which -h
rustup-which
显示给定命令将运行的二进制文件

使用方法：
    rustup which [选项] <命令>

参数：
    <命令>

选项：
        --toolchain <工具链>    工具链名称，例如 'stable'、'nightly' 或 '1.8.0'。
                            更多信息请参见 `rustup help toolchain`
    -h, --help                     打印帮助信息
</pre>

## rustup doc -h

<pre>
~> rustup doc -h
rustup-doc
打开当前工具链的文档

使用方法：
    rustup doc [选项] [主题]

参数：
    <主题>    主题，例如 'core', 'fn', 'usize', 'eprintln!', 'core::arch', 'alloc::format!', 'std::fs',
            'std::fs::read_dir', 'std::io::Bytes', 'std::iter::Sum', 'std::io::error::Result' 等...

选项：
        --path                     仅打印文档的路径
        --toolchain <工具链>        工具链名称，例如 'stable'、'nightly' 或 '1.8.0'。更多信息请参见
                                   `rustup help toolchain`
        --alloc                    Rust 核心分配和集合库
        --book                     《Rust 编程语言》书籍
        --cargo                    《Cargo》书籍
        --core                     Rust 核心库
        --edition-guide            Rust 版本指南
        --nomicon                  高级和未安全 Rust 编程的黑暗艺术
        --proc_macro               宏作者定义新宏时的支持库
        --reference                Rust 参考手册
        --rust-by-example          一系列可运行的示例，展示了各种 Rust 概念和标准库
        --rustc                    Rust 编程语言的编译器
        --rustdoc                  Rust 项目的文档生成器
        --std                      标准库 API 文档
        --test                     rustc 内置单元测试和微基准测试框架的支持代码
        --unstable-book            《不稳定书籍》
        --embedded-book            《嵌入式 Rust》书籍
    -h, --help                     打印帮助信息

讨论：
    使用默认浏览器打开当前活动工具链的文档。

    默认情况下，它会打开文档索引。使用各种标志来打开特定的文档部分。
</pre>

<p>&nbsp;</p>

## rustup self -h

<pre>
~> rustup self -h
rustup-self
修改 rustup 安装

使用方法：
    rustup self <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    update          下载并安装 rustup 的更新
    uninstall       卸载 rustup。
    upgrade-data    升级内部数据格式。
    help            打印此消息或给定子命令的帮助信息
</pre>

### rustup self update -h

<pre>
~> rustup self update -h
rustup-self-update
Download and install updates to rustup

USAGE:
    rustup self update

OPTIONS:
    -h, --help    Print help information
</pre>

### rustup self uninstall -h

<pre>
~> rustup self uninstall -h
rustup-self-uninstall
Uninstall rustup.

USAGE:
    rustup self uninstall [OPTIONS]

OPTIONS:
    -y
    -h, --help    Print help information
</pre>

### rustup self upgrade-data -h

<pre>
~> rustup self upgrade-data -h
rustup-self-upgrade-data
Upgrade the internal data format.

USAGE:
    rustup self upgrade-data

OPTIONS:
    -h, --help    Print help information
</pre>

<p>&nbsp;</p>

## rustup set -h

<pre>
~> rustup set -h
rustup-set
修改 rustup 设置

使用方法：
    rustup set <子命令>

选项：
    -h, --help    打印帮助信息

子命令：
    default-host        当未指定时，用于识别工具链的三重奏
    profile             默认安装的组件
    auto-self-update    rustup 自动自我更新模式
    help                打印此消息或给定子命令的帮助信息
</pre>

### rustup set default-host -h

<pre>
~> rustup set default-host -h
rustup-set-default-host
The triple used to identify toolchains when not specified

USAGE:
    rustup set default-host <host_triple>

ARGS:
    <host_triple>

OPTIONS:
    -h, --help    Print help information
</pre>

### rustup set profile -h

<pre>
~> rustup set profile -h
rustup-set-profile
The default components installed

USAGE:
    rustup set profile <profile-name>

ARGS:
    <profile-name>    [default: default] [possible values: minimal, default, complete]

OPTIONS:
    -h, --help    Print help information
</pre>

### rustup set auto-self-update -h

<pre>
~> rustup set auto-self-update -h
rustup-set-auto-self-update
The rustup auto self update mode

USAGE:
    rustup set auto-self-update <auto-self-update-mode>

ARGS:
    <auto-self-update-mode>    [default: enable] [possible values: enable, disable, check-only]

OPTIONS:
    -h, --help    Print help information
</pre>

<p>&nbsp;</p>

## rustup completions -h

<pre>
~> rustup completions -h
rustup-completions
为您的 shell 生成 tab 补全脚本

使用方法：
    rustup completions [参数]

参数：
    <shell>      [可能的值：bash, elvish, fish, powershell, zsh]
    <command>    [可能的值：rustup, cargo]

选项：
    -h, --help    打印帮助信息

讨论：
    启用 Bash、Fish、Zsh 或 PowerShell 的 tab 补全
    脚本输出到 `stdout`，允许将其重定向到您选择的文件。
    您放置文件的位置将取决于您使用的 shell 和操作系统。您的特定配置也可能决定这些脚本需要放在哪里。

    这里是一些在 Unix 和类似操作系统（如 GNU/Linux）下支持的三种 shell 的常见设置。

    BASH：

    补全文件通常存储在 `/etc/bash_completion.d/` 用于系统级命令，
    但可以存储在 `~/.local/share/bash-completion/completions` 用于用户特定命令。运行以下命令：

        $ mkdir -p ~/.local/share/bash-completion/completions

        $ rustup completions bash >> ~/.local/share/bash-completion/completions/rustup

    这安装了补全脚本。您可能需要退出并重新登录您的 shell 会话才能使更改生效。

    BASH (macOS/Homebrew)：

    Homebrew 将 bash 补全文件存储在 Homebrew 目录内。
    安装了 `bash-completion` brew 公式后，运行以下命令：

        $ mkdir -p $(brew --prefix)/etc/bash_completion.d

        $ rustup completions bash > $(brew --prefix)/etc/bash_completion.d/rustup.bash-completion

    FISH：

    Fish 补全文件通常存储在 `$HOME/.config/fish/completions`。运行以下命令：

        $ mkdir -p ~/.config/fish/completions

        $ rustup completions fish > ~/.config/fish/completions/rustup.fish

    这安装了补全脚本。您可能需要退出并重新登录您的 shell 会话才能使更改生效。

    ZSH：

    Zsh 补全通常存储在 `$fpath` 变量列出的任何目录中。
    要使用这些补全，您必须将生成的脚本添加到这些目录之一，或者将您自己的添加到此列表。

    添加自定义目录通常是最安全的赌注，如果您不确定要使用哪个目录。
    首先创建目录；在这个例子中，我们将在 `$HOME` 目录中创建一个隐藏目录：

        $ mkdir ~/.zfunc

    然后将以下行添加到您的 `.zshrc` 中，位于 `compinit` 之前：

        fpath+=~/.zfunc

    现在您可以使用以下命令安装补全脚本：

        $ rustup completions zsh > ~/.zfunc/_rustup

    您必须然后退出并重新登录，或者简单地运行：

        $ exec zsh

    以使新的补全生效。

    自定义位置：

    或者，您可以把这些文件保存到您选择的地方，比如 `$HOME` 中的自定义目录。
    这样做需要您添加适当的指令，比如在登录脚本中 `source`。咨询您的 shell 文档以了解如何添加此类指令。

    POWERSHELL：

    PowerShell 补全脚本需要 PowerShell v5.0+（它随 Windows 10 提供，但可以单独下载用于 Windows 7 或 8.1）。
    首先，检查是否已经设置了配置文件

        PS C:\> Test-Path $profile

    如果上述命令返回 `False`，则运行以下命令：

        PS C:\> New-Item -path $profile -type file -force

    现在打开由 `$profile` 提供的文件（如果您使用了 `New-Item` 命令，它将是
    `${env:USERPROFILE}\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`）。

    接下来，我们将补全文件保存到我们的配置文件中，或者保存到一个单独的文件并在我们的配置文件中引用它。
    要将补全保存到我们的配置文件中，只需使用：

        PS C:\> rustup completions powershell >>
        ${env:USERPROFILE}\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1

    CARGO：

    Rustup 还可以为 `cargo` 生成一个补全脚本。由 `rustup` 输出的脚本将引用与您的默认工具链一起分发的补全脚本。
    并非所有 shell 都目前支持。以下是当前支持 shell 的示例。

    BASH：

        $ rustup completions bash cargo >> ~/.local/share/bash-completion/completions/cargo

    ZSH：

        $ rustup completions zsh cargo > ~/.zfunc/_cargo
</pre>
