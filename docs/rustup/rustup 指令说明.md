
# rustup -h

<pre><code>
~> rustup -h

rustup 1.26.0 (5af9b9484 2023-04-05)
The Rust toolchain installer

使用方法：
    ~> rustup [选项] [+toolchain] <+子命令>

参数：
    <+toolchain>    发布渠道（例如 +stable）或自定义工具链以设置覆盖

选项：
    -v, --verbose    启用详细输出
    -q, --quiet      禁用进度输出
    -h, --help       打印帮助信息
    -V, --version    打印版本信息

子命令：
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;show           显示活动和解压的工具链或配置文件</strong></summary>
    ~> rustup show -h
    rustup-show
    显示活动工具链和安装的工具链或配置文件

    使用方法：
        rustup show [选项] [子命令]

    选项：
        -v, --verbose    启用带有 rustc 信息的详细输出，用于所有安装的工具链
        -h, --help       打印帮助信息

    子命令：
        <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;active-toolchain    显示活动工具链</strong></summary>
            沾污扩展
        </details>
        <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;home                显示计算得出的 RUSTUP_HOME 值</strong></summary>
            沾污扩展
        </details>
        <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;profile             显示当前配置文件</strong></summary>
            沾污扩展
        </details>
        <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;help                打印此消息或给定子命令的帮助信息</strong></summary>
            沾污扩展
        </details>

    说明：
        显示活动工具链的名称和 rustc 的版本。

        如果活动工具链安装了支持其他编译目标的支持，那么它们也会被列出。

        如果安装了多个工具链，那么所有安装的工具链也会被列出。

    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;update         更新 Rust 工具链和 rustup</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;check          检查 Rust 工具链和 rustup 的更新</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;default        设置默认工具链</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;toolchain      修改或查询已安装的工具链</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;target         修改工具链支持的目标</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;component      修改工具链安装的组件</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;override       修改目录工具链覆盖</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;run            使用为给定工具链配置的环境运行命令</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;which          显示将为给定命令运行的二进制文件</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;doc            打开当前工具链的文档</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;self           修改 rustup 安装</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;set            修改 rustup 设置</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;completions    为您的 shell 生成制表完成脚本</strong></summary>
        沾污扩展
    </details>
    <details><summary style="line-height: 0"><strong>&nbsp;&nbsp;&nbsp;help           打印此消息或给定子命令的帮助信息</strong></summary>
        沾污扩展
    </details>

说明：
    Rustup 是从 Rust 程序设计语言官方发布渠道安装 ，使您可以轻松地在稳定版、测试版和夜间版编译器之间切换并保持更新。
    它通过为常见平台的标准库提供二进制构建，使交叉编译变得简单。

    如果您是Rust的新手，请考虑运行 `rustup doc --book` 来学习Rust。

示例:

    列出当前目录中的可见文件
    > ls

    列出子目录中的可见文件
    > ls subdir

    列出父目录中的可见文件的完整路径
    > ls -f ..

    列出 Rust 文件
    > ls *.rs

    列出名称不包含 'bar' 的文件和目录
    > ls -s | where name !~ bar

    列出主目录中的所有目录
    > ls -a ~ | where type == dir

    列出主目录中在 7 天内未被修改的所有目录
    > ls -as ~ | where type == dir and modified < ((date now) - 7day)

    列出给定路径并显示目录本身
    > ['/path/to/directory' '/path/to/file'] | each {|| ls -D $in } | flatten

</code></pre>

<style type="text/css">
    details > summary {
        display: flex;
        align-items: center; /* 这将使得子元素在交叉轴上居中对齐 */
        padding-right: 10px; /* 这将为右侧的图标提供一些空间 */
        margin-left: 10px; /* 这将使得文本靠左一些 */
    }

    details > summary::before {
      content: "✨";
    }
  
    details > summary::-webkit-details-marker {
        float: right; /* 这将使得图标靠右 */
    }

    pre > code > details {
        margin-bottom: -20px; /* 设置标签底部边距 */
    }

    pre > code > details > details {
        margin-bottom: -20px;
    }
</style>
