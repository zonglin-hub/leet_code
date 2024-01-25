# 安装 OpenSHH

注意：卸载完成服务器ssh服务就会停掉，此时客户端连接的不能断开，在ssh没安装好之前也不可重启ssh服务和服务器，否则断开之后将无法进行远程连接

参考文档：

- [centos7 升级openssh到openssh-8.0p1版本](https://www.cnblogs.com/nmap/p/10779658.html)
- [shell 升级 openssl-1.1.1n openssh-8.9p1](https://www.cnblogs.com/liuzonglin/p/16202966.html)

---

**安装包下载地址：**

- [openssl 下载](https://ftp.openssl.org/source/)
- [OpenSSH 下载](https://cdn.openbsd.org/pub/OpenBSD/OpenSSH/portable/)

使用 `ssh -v` 查看 ssh 版本

## 上传 openssh 解压安装包

```sh
tar -zxvf openssh-8.8p1.tar.gz; cd openssh-8.8p1/
```

## 删除原先 ssh 的配置文件和目录

```sh
rm -rf /etc/ssh/*
```

## 配置、编译、安装

```sh
yum -y install zlib zlib-devel
yum install -y openssl-devel
yum -y install pam-devel
./configure --prefix=/usr/ --sysconfdir=/etc/ssh  --with-openssl-includes=/usr/local/ssl/include --with-ssl-dir=/usr/local/ssl --with-zlib --with-md5-passwords --with-pam && make && make install
echo $?
```

以上命令执行完毕，`echo $?`查看下最后的make install是否有报错，0表示没有问题

## 修改配置文件最终内容如下，其他的不要动 /etc/ssh/sshd_config 并重启

```sh
grep "^PermitRootLogin" /etc/ssh/sshd_config
grep "UseDNS" /etc/ssh/sshd_config
grep "UsePAM" /etc/ssh/sshd_config
```

```sh
systemctl restart sshd     # 重启sshd
```

## 从原先的解压的包中拷贝一些文件到目标位置（如果目标目录存在就覆盖）

```sh
cp -a contrib/redhat/sshd.init /etc/init.d/sshd
cp -a contrib/redhat/sshd.pam /etc/pam.d/sshd.pam
chmod +x /etc/init.d/sshd
chkconfig --add sshd
systemctl enable sshd
/etc/init.d/sshd restart
```

## 把原先的systemd管理的sshd文件删除或者移走或者删除，不移走的话影响我们重启sshd服务

```sh
mv  /usr/lib/systemd/system/sshd.service  /
```

## 设置sshd服务开机启动

```sh
chkconfig sshd on
```

## 测试启动停止服务。都正常

```sh
以后管理sshd通过下面方式了
[root@linux-node3 ~]# /etc/init.d/sshd restart
Restarting sshd (via systemctl):                           [  OK  ]
[root@linux-node3 ~]# netstat -lntp
Active Internet connections (only servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name   
tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN      31800/sshd         
tcp6       0      0 :::22                   :::*                    LISTEN      31800/sshd         
tcp6       0      0 :::23                   :::*                    LISTEN      1/systemd          
[root@linux-node3 ~]# /etc/init.d/sshd stop
Stopping sshd (via systemctl):                             [  OK  ]
[root@linux-node3 ~]# netstat -lntp
Active Internet connections (only servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name   
tcp6       0      0 :::23                   :::*                    LISTEN      1/systemd          
[root@linux-node3 ~]# /etc/init.d/sshd start
Starting sshd (via systemctl):                            [  OK  ]
[root@linux-node3 ~]#
```

## 使用systemd方式也行

```sh
[root@linux-node3 ~]# systemctl stop sshd
[root@linux-node3 ~]# netstat -lntp
Active Internet connections (only servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name   
tcp6       0      0 :::23                   :::*                    LISTEN      1/systemd          
[root@linux-node3 ~]# systemctl start sshd
[root@linux-node3 ~]# netstat -lntp
Active Internet connections (only servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name   
tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN      31958/sshd         
tcp6       0      0 :::22                   :::*                    LISTEN      31958/sshd         
tcp6       0      0 :::23                   :::*                    LISTEN      1/systemd          
[root@linux-node3 ~]# systemctl restart sshd
[root@linux-node3 ~]# netstat -lntp
Active Internet connections (only servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name   
tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN      31999/sshd         
tcp6       0      0 :::22                   :::*                    LISTEN      31999/sshd         
tcp6       0      0 :::23                   :::*                    LISTEN      1/systemd          
[root@linux-node3 ~]#
```

## 测试版本。都正常

使用 ssh -v 测试

## OpenSSH 启动

`systemctl restart sshd` #一定要重启

`systemctl status sshd` #再次检查

xshell ssh 协议连接密码连接：SHH服务器拒绝了密码。请再试一次。
修改：vim /etc/ssh/sshd_config

```shell
PermitRootLogin yes
UsePAM yes
UseDNS no
```
