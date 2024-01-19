df - 报告文件系统磁盘空间的使用情况

       df [OPTION]... [FILE]...

       POSIX 选项: [-kP]

       GNU 选项 (最短方式): [-ahHiklmPv] [-t fstype] [-x fstype] [--block-size=size] [--print-type] [--no-sync] [--sync] [--help] [--version] [--]

描述：
       此手册页文档是df的GNU版本. df命令列出指定的每一个文件名所在的文件系统上可用磁盘空间的数量。
    如果没有指定文件名,则显示当前所有使用中的文件系统.缺省设置时,磁盘空间以1K为一块显示,如果环境变量POSIXLY_CORRECT已设
       置,则采用512字节为一块显示. 如果参数是一个包含已使用文件系统的磁盘设备名, df命令显示出的是该文件系统的可用空间,而非包含设备结点的文件系统(只能是根文件系统).此  版本的df不能显示未使用文件系统的可用空间,这是由于大多数系统在响应这样的请求时必须很清楚该文件系统的结构。

选项
       -a, --all
              列出包括BLOCK为0的文件系统

       --block-size=SIZE use SIZE-byte blocks
              指定块的大小

       -h,--huma-readable"
              用常见的格式显示出大小(例如:1K 234M 2G)

       -H,--si"
              同上,但是这里的1k等于1000字节而不是1024字节

       -i, --inodes
              用信息索引点代替块表示使用状况


       -k, --kilobytes
              指定块大小等于1024字节来显示使用状况

       -l, --local
              只显示本地文件系统使用状况

       -m, --megabytes
              以指定块大小等于1048576字节(1M)来显示使用状况

       --no-sync
              在取得使用信息前禁止调用同步 (default)

       -P, --portability
              使用POSIX格式输出

       --sync 在取得使用信息前调用同步

       -t, --type=TYPE
              只显示指定类型(TYPE)的文件系统

       -T, --print-type
              输出每个文件系统的类型

       -x, --exclude-type=TYPE
              只显示指定类型(TYPE)之外的文件系统.

       -v (忽

       --     输出该命令的帮助信息并退出

       --version
              输出版本信息并退出
