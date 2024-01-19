[tar命令 – 压缩和解压缩文件 – Linux命令大全(手册) (linuxcool.com)](https://www.linuxcool.com/tar)

---

**语法格式：** tar 参数 文件或目录

**常用参数：**

| 参数          | 解释                                        |
| --------------- | --------------------------------------------- |
| -z            | 通过gzip指令压缩/解压缩文件，文件名最好为`*.tar.gz`<br /> |
| -x            | 从归档中解出文件                            |
| -v            | 详细地列出处理的文件                        |
| -f <备份文件> | 指定备份文件                                |
| -c            | 建立新的备份文件<br />                          |
| -C <目录>     | 仅压缩指定目录里的内容或解压缩到指定目录<br />  |

**参考案例：**

```shell
tar -zxvf install_20220105.tar.gz    # 解压文件
tar -zxvf install_20220105.tar.gz -C /etc   # 解压到etc目录下 
tar -zcvf install_20220105.tar.gz install_20220105  # 压缩文件
```
