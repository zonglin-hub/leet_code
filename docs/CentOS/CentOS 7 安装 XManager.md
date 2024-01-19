# CentOS7 安装 XManager

```sh
yum install -y epel-release         # 安装epel源
```

```sh
yum install -y lightdm && yum groupinstall -y xfce        # 安装lightdm和Xfce
```

修改lightdm.conf文件
`vim /etc/lightdm/lightdm.conf

```roboconf
[XDMCPServer]
enabled=true
port=177
```

```sh
systemctl disable gdm && systemctl enable lightdm     # 将Display Manager切换为lightdm
```

```sh
systemctl start lightdm     # 启动lightdm
```

```sh
sudo systemctl stop firewalld      # 关闭防火墙
```

参考博客：
使用X Manager远程CentOS 7服务器（XDMCP）：<https://blog.csdn.net/lyfqyr/article/details/89041929>
