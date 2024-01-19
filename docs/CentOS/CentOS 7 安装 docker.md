## CentOS 7 安装卸载 docker

### 查看内核版本，需要3.10以上

```sh
uname -r
```

### 更新 yum 软件包索引

```sh
yum makecache fast
```

### 卸载旧版本

```sh
yum remove docker \
                  docker-client \
                  docker-client-latest \
                  docker-common \
                  docker-latest \
                  docker-latest-logrotate \
                  docker-logrotate \
                  docker-engine
```

### 安装工具包

```sh
yum install -y yum-utils
```

### 设置镜像厂库

```sh
yum-config-manager \
    --add-repo \
    http://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo
```

### 安装 docker Engine-Community

```sh
yum install docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

### 启动 docker

```sh
systemctl start docker
```

### 判断是否装成功

```sh
docker version
```

## 阿里云镜像加速

[Docker配置阿里云镜像加速_@noNo的博客-CSDN博客_阿里云docker镜像加速](https://blog.csdn.net/m0_46665077/article/details/124248727)

## 卸载 docker

### 卸载 docker 依赖

```sh
yum remove docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

### 删除资源

```sh
rm -rf /var/lib/docker
```

## centos7 设置 docker 开机自启

```sh
systemctl enable docker 
```

### 执行完毕需要重启系统

```sh
init 6
```

## [CentOS7 安装 docker-compose](https://www.cnblogs.com/liuzonglin/p/17199465.html)
