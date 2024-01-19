```shell
[root@FortA ]# vim /etc/sysconfig/network-scripts/ifcfg-ens32  # 修改网卡配置信息
TYPE="Ethernet"
PROXY_METHOD="none"
BROWSER_ONLY="no"
BOOTPROTO="dhcp"
DEFROUTE="yes"
IPV4_FAILURE_FATAL="no"
IPV6INIT="yes"
IPV6_AUTOCONF="yes"
IPV6_DEFROUTE="yes"
IPV6_FAILURE_FATAL="no"
IPV6_ADDR_GEN_MODE="stable-privacy"
NAME="ens33"
UUID="9f4169e5-3891-48d1-a60a-e8e55e1e2e1a"
DEVICE="ens33"
ONBOOT="yes"        # 修改 ONBOOT=yes
```

重启网络服务

```shell
systemctl restart network
```

测试

```shell
ping www.baidu.com
```
