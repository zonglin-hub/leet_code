## 网卡配置格式

<https://www.cnblogs.com/liuzonglin/p/15743377.html>

```sh
[root@Fort network-scripts]# ip a # 查看网卡
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: GE1-0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:58:43 brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.90/18 brd 192.168.63.255 scope global GE1-0
       valid_lft forever preferred_lft forever
3: GE1-1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:58:44 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.90/18 brd 192.168.63.255 scope global GE1-1
       valid_lft forever preferred_lft forever
4: GE1-2: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:58:45 brd ff:ff:ff:ff:ff:ff
    inet 192.168.2.90/18 brd 192.168.63.255 scope global GE1-2
       valid_lft forever preferred_lft forever
5: GE1-3: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:58:46 brd ff:ff:ff:ff:ff:ff
    inet 192.168.3.90/18 brd 192.168.63.255 scope global GE1-3
       valid_lft forever preferred_lft forever
6: GE0-0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP group default qlen 1000
    link/ether 8c:1c:da:43:11:97 brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.91/18 brd 192.168.63.255 scope global GE0-0
       valid_lft forever preferred_lft forever
    inet6 fc00:bb04:1868:1:8e1c:daff:fe43:1197/64 scope global mngtmpaddr dynamic 
       valid_lft forever preferred_lft forever
    inet6 fe80::8e1c:daff:fe43:1197/64 scope link 
       valid_lft forever preferred_lft forever
7: GE0-1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:11:98 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.91/18 brd 192.168.63.255 scope global GE0-1
       valid_lft forever preferred_lft forever
8: GE0-2: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:11:99 brd ff:ff:ff:ff:ff:ff
    inet 192.168.2.91/18 brd 192.168.63.255 scope global GE0-2
       valid_lft forever preferred_lft forever
9: GE0-3: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:11:9a brd ff:ff:ff:ff:ff:ff
    inet 192.168.3.91/18 brd 192.168.63.255 scope global GE0-3
       valid_lft forever preferred_lft forever
10: GE0-4: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:11:9b brd ff:ff:ff:ff:ff:ff
    inet 192.168.4.91/18 brd 192.168.63.255 scope global GE0-4
       valid_lft forever preferred_lft forever
11: GE0-5: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc mq state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:11:9c brd ff:ff:ff:ff:ff:ff
    inet 192.168.5.91/18 brd 192.168.63.255 scope global GE0-5
       valid_lft forever preferred_lft forever
12: enp2s0f0: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:56:71 brd ff:ff:ff:ff:ff:ff
13: enp2s0f1: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN group default qlen 1000
    link/ether 8c:1c:da:43:56:72 brd ff:ff:ff:ff:ff:ff
[root@Fort network-scripts]# 
[root@Fort network-scripts]# pwd # 配置网卡路径
/etc/sysconfig/network-scripts
[root@Fort network-scripts]# 
[root@Fort network-scripts]# 
[root@Fort network-scripts]# 
[root@Fort network-scripts]# 
[root@Fort network-scripts]# cat ifcfg-GE0-0 # 所有网卡配置以此类推
TYPE=Ethernet
PROXY_METHOD=none
BROWSER_ONLY=no
BOOTPROTO=static
DEFROUTE=yes
IPV4_FAILURE_FATAL=no
IPV6INIT=yes
IPV6_AUTOCONF=yes
IPV6_DEFROUTE=yes
IPV6_FAILURE_FATAL=no
IPV6_ADDR_GEN_MODE=stable-privacy
NAME=GE0-0 # 修改
DEVICE=GE0-0 # 修改
HWADDR=8c:1c:da:43:11:97 # mac地址 ip a 
ONBOOT=yes
IPADDR=192.168.0.91
NETWORK=255.255.255.0
NETMASK=255.255.192.0
GATEWAY=192.168.1.1
DNS1=114.114.114.114
DNS2=
[root@Fort network-scripts]# 
```

![image](https://img2022.cnblogs.com/blog/2402369/202202/2402369-20220223233341875-1677842879.png)
![image](https://img2022.cnblogs.com/blog/2402369/202202/2402369-20220223233509599-595593188.png)

# 注意事项

单机模式下相同网段是可以配置多个网卡；避免问题多个网卡最号不要配置相同网段
重启系统，修改网卡名是需要重启系统的！

# 参考文档

<https://www.cnblogs.com/hanshanxiaoheshang/p/9433504.html>
