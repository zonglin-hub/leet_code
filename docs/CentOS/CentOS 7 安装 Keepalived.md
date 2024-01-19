### 安装Keepalived

#### 编译安装

下载地址

[Keepalived for Linux](https://www.keepalived.org/download.html#)

安装依赖

```shell
yum install openssl-devel
```

使用 ./configure 编译安装

如遇报错提示

```shell
configure: error:
!!! OpenSSL is not properly installed on your system. !!!
!!! Can not include OpenSSL headers files. !!!

```

#### yum 安装

```shell
yum install keepalived

```

### 配置

使用yum安装后配置文件在

```shell
/etc/keepalived/keepalived.conf
```

#### 最小配置

第一台机器

```shell
! Configuration File for keepalived
global_defs {
    router_id lb111
}
vrrp_instance atguigu {
    state MASTER
    interface ens33
    virtual_router_id 51
    priority 100
    advert_int 1
    authentication {
        auth_type PASS
        auth_pass 1111
    }
    virtual_ipaddress {
        192.168.44.200
    }
}
```

第二台机器

```shell
! Configuration File for keepalived
global_defs {
    router_id lb110
}
vrrp_instance atguigu {
    state BACKUP
    interface ens33
    virtual_router_id 51
    priority 50
    advert_int 1
    authentication {
        auth_type PASS
        auth_pass 1111
    }
    virtual_ipaddress {
        192.168.44.200
    }
}
```

启动服务

```shell
systemctl start keepalived
```
