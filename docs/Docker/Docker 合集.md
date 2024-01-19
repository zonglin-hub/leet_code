# CentOS 7 安装卸载 docker

```bash
# 查看内核版本，需要3.10以上
uname -r

# 更新 yum 软件包索引
yum makecache fast

# 卸载旧版本
yum remove docker \
                  docker-client \
                  docker-client-latest \
                  docker-common \
                  docker-latest \
                  docker-latest-logrotate \
                  docker-logrotate \
                  docker-engine

# 安装工具包
yum install -y yum-utils

# 设置镜像厂库
yum-config-manager \
    --add-repo \
    http://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo

# 安装 docker Engine-Community
yum install docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 启动 docker
systemctl start docker

# 判断是否安装成功
docker version

# 测试
docker run hello-world

# 查看 hello-world
docker images
```

## 阿里云镜像加速

[Docker配置阿里云镜像加速](https://blog.csdn.net/m0_46665077/article/details/124248727 "Docker配置阿里云镜像加速")

## 卸载 docker 依赖

```bash
# 卸载 docker 依赖
yum remove docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 删除资源
rm -rf /var/lib/docker
```

# centos 7 设置 docker 开机自启服务

```bash
[root@localhost init.d]# systemctl list-unit-files # 查看开机自启项列表
UNIT FILE                                     STATE   
proc-sys-fs-binfmt_misc.automount             static  
dev-hugepages.mount                           static  
dev-mqueue.mount                              static  
proc-sys-fs-binfmt_misc.mount                 static  
run-vmblock\x2dfuse.mount                     disabled
sys-fs-fuse-connections.mount                 static  
...
[root@localhost init.d]# systemctl list-unit-files | grep docker # 查询状态筛选服务
docker.service                                disabled
docker.socket                                 disabled
[root@localhost init.d]# systemctl enable docker # 设置docker服务开机启动
Created symlink from /etc/systemd/system/multi-user.target.wants/docker.service to /usr/lib/systemd/system/docker.service.
[root@localhost init.d]# systemctl list-unit-files | grep docker
docker.service                                enabled 
docker.socket                                 disabled
[root@localhost init.d]# init 6 # 重启系统
Connection closing...Socket close.

Connection closed by foreign host.

Disconnected from remote host(192.168.1.100) at 18:21:48.
[root@localhost ~]#  systemctl status docker # 查看 docker 状态
● docker.service - Docker Application Container Engine
   Loaded: loaded (/usr/lib/systemd/system/docker.service; enabled; vendor preset: disabled)
   Active: active (running) since 二 2022-10-04 18:22:16 CST; 53s ago
     Docs: https://docs.docker.com
 Main PID: 1319 (dockerd)
    Tasks: 11
   Memory: 96.2M
   CGroup: /system.slice/docker.service
           └─1319 /usr/bin/dockerd -H fd:// --containerd=/run/containerd/containerd.sock

10月 04 18:22:15 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:15.702633344..."
10月 04 18:22:15 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:15.745525782..."
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.055185788..."
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.203268252..."
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.291487816..."
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.378593826..."
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.441026971...8
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.441153515..."
10月 04 18:22:16 localhost.localdomain systemd[1]: Started Docker Application Container ...e.
10月 04 18:22:16 localhost.localdomain dockerd[1319]: time="2022-10-04T18:22:16.461010934..."
Hint: Some lines were ellipsized, use -l to show in full.
[root@localhost ~]# 
```

**参数描述：**

- UNIT FILE : 服务名称
- STATE : 状态
  - enabled : 开机启动
  - disabled : 开机不启动

## 运行容器开启容器自启

```sh
docker update --restart=always 356c575a09aa
```

## 参考文档

[centos7中设置服务开机自启的两种方法 - 简书 (jianshu.com)](https://www.jianshu.com/p/6e4ad5b19144)

# docker 常用命令

```bash
# 帮助指令
docker [命令] --help
# 显示 docker 版本信息
docker version
# 显示 docker 系统信息
docker info
```

## docker 镜像使用

```bash
# 查看所有镜像id
docker images
# 查找镜像
docker search centos
# 下载镜像
docker pull centos
# 删除指定id容器
docker rmi -f feb5d9fea6a5
# 启动镜像并进入容器
docker run -it centos /bin/bash
  -i # 交互式操作
  -t # 终端
  /bin/bash # 交互式 Shell
# 停止并退出容器
exit
# 容器不停止退出
Ctrl + P + Q
# 查看运行容器
docker ps
# 删除容器
docker rm feb5d9fea6a5
# 启动
docker start feb5d9fea6a5
# 重启
docker restart feb5d9fea6a5
# 停止
docker stop feb5d9fea6a5
docker kill feb5d9fea6a5
# 通过镜像启动容器，容器停止了
docker run -d centos
# 查看容器运行日志
docker logs -ft --tail 10  9970a5ec9fb5
# 进入一个新的终端
docker exec -it 3913ce5ec9fd /bin/bash
# 进入当前正在运行的容器
docker attach 3913ce5ec9fd
# 清理所有运行容器
docker rm -f $(docker ps -qa)
 参数说明：
 --all , -a      # 列出所有镜像
 --quiet , -q    # 只显示镜像的id
```

<details>
<summary>点击查看代码</summary>

```bash
# 显示 docker 版本信息
docker version 
# 显示 docker 系统信息
docker info 
# 帮助命令
docker [命令] -help 
# 查看所有镜像id
docker images -aq
# 收索镜像
docker search mysql
# 下载镜像
docker pull mysql
# 删除指定id容器
docker rmi -f feb5d9fea6a5 
# 下载 centos 镜像
docker pull centos
# 新建容器并启动
# 启动镜像并进入容器
docker run -it centos /bin/bash
# 停止并退出容器
exit
# 容器不停止退出
Ctrl + P + Q
# 查看运行容器
docker ps
  -a # 展示出所有运行过的容器
        -n=? # 显示最近创建的容器
        -q # 显示容器编号
# 删除容器
docker rm feb5d9fea6a5
# 启动
docker start feb5d9fea6a5
# 重启
docker restart feb5d9fea6a5
# 停止
docker stop feb5d9fea6a5
docker kill feb5d9fea6a5

# 通过镜像启动容器，容器停止了
docker run -d centos
# 查看容器运行日志
docker logs -f -t --tail 10  9970a5ec9fb5
[root@localhost ~]# docker run -d centos /bin/bash -c "while true; do echo liuzonglin; sleep 1; done"
3913ce5ec9fd2ed91e93df0c2088485d5fbce5e75ed630d4f7b64aac357ac51f
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND                  CREATED          STATUS          PORTS     NAMES
3913ce5ec9fd   centos    "/bin/bash -c 'while…"   27 seconds ago   Up 27 seconds             admiring_haslett
9970a5ec9fb5   centos    "/bin/bash"              17 minutes ago   Up 17 minutes             clever_jennings
[root@localhost ~]# docker logs -ft --tail 10 3913ce5ec9fd
2022-10-01T15:18:18.647064020Z liuzonglin
2022-10-01T15:18:19.651083452Z liuzonglin
2022-10-01T15:18:20.655981698Z liuzonglin
2022-10-01T15:18:21.659626269Z liuzonglin
2022-10-01T15:18:22.665580566Z liuzonglin
^C
[root@localhost ~]# 
[root@localhost ~]# docker top 3913ce5ec9fd # 查看容器进程信息
UID                 PID                 PPID                C                   STIME               TTY                 TIME                CMD
root                72577               72559               0                   23:17               ?                   00:00:00            /bin/bash -c while true; do echo liuzonglin; sleep 1; done
root                73010               72577               0                   23:23               ?                   00:00:00            /usr/bin/coreutils --coreutils-prog-shebang=sleep /usr/bin/sleep 1
[root@localhost ~]# 
[root@localhost ~]# docker inspect 3913ce5ec9fd # 查看容器元数据
[
    {
        "Id": "3913ce5ec9fd2ed91e93df0c2088485d5fbce5e75ed630d4f7b64aac357ac51f",
        "Created": "2022-10-01T15:17:48.112633999Z",
        "Path": "/bin/bash",
        "Args": [
            "-c",
            "while true; do echo liuzonglin; sleep 1; done"
        ],
        "State": {
            "Status": "running",
            "Running": true,
            "Paused": false,
            "Restarting": false,
            "OOMKilled": false,
            "Dead": false,
            "Pid": 72577,
            "ExitCode": 0,
......
[root@localhost ~]# 
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND                  CREATED          STATUS          PORTS     NAMES
3913ce5ec9fd   centos    "/bin/bash -c 'while…"   11 minutes ago   Up 11 minutes             admiring_haslett
9970a5ec9fb5   centos    "/bin/bash"              29 minutes ago   Up 29 minutes             clever_jennings
[root@localhost ~]# 
[root@localhost ~]# 
[root@localhost ~]# 
[root@localhost ~]# 
[root@localhost ~]# docker exec -it 3913ce5ec9fd /bin/bash # 进入一个新的终端
[root@3913ce5ec9fd /]# 
[root@3913ce5ec9fd /]# 
[root@3913ce5ec9fd /]# 
[root@3913ce5ec9fd /]# exit;
exit
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND                  CREATED          STATUS          PORTS     NAMES
3913ce5ec9fd   centos    "/bin/bash -c 'while…"   13 minutes ago   Up 13 minutes             admiring_haslett
9970a5ec9fb5   centos    "/bin/bash"              30 minutes ago   Up 30 minutes             clever_jennings
[root@localhost ~]# docker attach 3913ce5ec9fd # 进入当前正在运行的容器
liuzonglin
liuzonglin
[root@localhost ~]# docker rm -f $(docker ps -qa) # 清理所有运行容器
3913ce5ec9fd
9970a5ec9fb5
a23efb86ac89
8c620d0bc309
79487abc709b
1aca4d59732d
5d8c99947416
b81b70d4a84e
7ae37a790506
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
[root@localhost ~]# docker images
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
redis        latest    7614ae9453d1   9 months ago    113MB
mysql        latest    3218b38490ce   9 months ago    516MB
centos       latest    5d0da3dc9764   12 months ago   231MB
[root@localhost ~]# 


```

</details>

[官网 docker 指令帮助文档](https://docs.docker.com/engine/reference/commandline/docker/ "官网 docker 指令帮助文档")

# **docker 安装 redis**

<details>
<summary>点击查看代码</summary>

```bash
[root@localhost ~]# docker pull redis:latest # 最新版本的镜像
latest: Pulling from library/redis
Digest: sha256:db485f2e245b5b3329fdc7eff4eb00f913e09d8feb9ca720788059fdc2ed8339
Status: Image is up to date for redis:latest
docker.io/library/redis:latest
[root@localhost ~]# docker images # 查看本地镜像
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
redis        latest    7614ae9453d1   9 months ago    113MB
[root@localhost bin]# docker run -itd --name redis-test -p 6379:6379 redis # 运行容器
70463da02fbb2437c288aa2259760280b23a7c884d75509ee73535f3aa963732
[root@localhost bin]# docker ps # 查看运行容器
CONTAINER ID   IMAGE     COMMAND                  CREATED          STATUS          PORTS                                       NAMES
70463da02fbb   redis     "docker-entrypoint.s…"   12 seconds ago   Up 11 seconds   0.0.0.0:6379->6379/tcp, :::6379->6379/tcp   redis-test
[root@localhost bin]# docker exec -it redis-test /bin/bash # 连接测试使用 redis 服务
root@70463da02fbb:/data# redis-cli # 连接 redis 客户端
127.0.0.1:6379> ping
PONG
127.0.0.1:6379> set test 1
OK
127.0.0.1:6379> get test
"1"
127.0.0.1:6379> 
```

参数说明：

- -p 6379:6379：映射容器服务的 6379 端口到宿主机的 6379 端口。外部可以直接通过宿主机ip:6379 访问到 Redis 的服务

</details>

# **docker 安装 mysql**

<details>
<summary>点击查看代码</summary>

```bash
[root@localhost ~]# docker pull mysql:latest # 最新版本的镜像
latest: Pulling from library/mysql
72a69066d2fe: Pull complete 
93619dbc5b36: Pull complete 
99da31dd6142: Pull complete 
626033c43d70: Pull complete 
37d5d7efb64e: Pull complete 
ac563158d721: Pull complete 
d2ba16033dad: Pull complete 
688ba7d5c01a: Pull complete 
00e060b6d11d: Pull complete 
1c04857f594f: Pull complete 
4d7cfa90e6ea: Pull complete 
e0431212d27d: Pull complete 
Digest: sha256:e9027fe4d91c0153429607251656806cc784e914937271037f7738bd5b8e7709
Status: Downloaded newer image for mysql:latest
docker.io/library/mysql:latest
[root@localhost ~]# docker images # 查看本地镜像
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
mysql        latest    3218b38490ce   9 months ago    516MB
[root@localhost ~]# docker run -itd --name mysql-test -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 mysqls # 运行容器
9a141aa3c4ce233500701e9d581c063f83158285ea7b6cd817f07cab2c7245aa
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND                  CREATED         STATUS         PORTS                                                  NAMES
29ac390af6dc   redis     "docker-entrypoint.s…"   4 seconds ago   Up 3 seconds   0.0.0.0:6379->6379/tcp, :::6379->6379/tcp              redis-test
9a141aa3c4ce   mysql     "docker-entrypoint.s…"   6 minutes ago   Up 6 minutes   0.0.0.0:3306->3306/tcp, :::3306->3306/tcp, 33060/tcp   mysql-test
[root@localhost ~]# docker exec -it mysql-test /bin/bash # 连接测试使用 mysql 服务
root@9a141aa3c4ce:/# mysql -h 127.0.0.1 -u root -p # 连接 mysql 后他
Enter password: 
Welcome to the MySQL monitor.  Commands end with ; or \g.
Your MySQL connection id is 8
Server version: 8.0.27 MySQL Community Server - GPL

Copyright (c) 2000, 2021, Oracle and/or its affiliates.

Oracle is a registered trademark of Oracle Corporation and/or its
affiliates. Other names may be trademarks of their respective
owners.

Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

mysql> show tables;
ERROR 1046 (3D000): No database selected
mysql> show databases;
+--------------------+
| Database           |
+--------------------+
| information_schema |
| mysql              |
| performance_schema |
| sys                |
+--------------------+
4 rows in set (0.01 sec)

mysql> 

```

参数说明：

latest 最新版本

- **-p 3306:3306** ：映射容器服务的 3306 端口到宿主机的 3306 端口，外部主机可以直接通过 **宿主机ip:3306** 访问到 MySQL 的服务。
- **MYSQL_ROOT_PASSWORD=123456**：设置 MySQL 服务 root 用户的密码。

</details>

# **docker 安装 mongo**

<details>
<summary>点击查看代码</summary>

```bash
[root@localhost ~]# docker pull mongo:latest
latest: Pulling from library/mongo
Digest: sha256:5be752bc5f2ac4182252d0f15d74df080923aba39700905cb26d9f70f39e9702
Status: Image is up to date for mongo:latest
docker.io/library/mongo:latest
[root@localhost ~]# docker images
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
redis        latest    7614ae9453d1   9 months ago    113MB
mysql        latest    3218b38490ce   9 months ago    516MB
mongo        latest    dfda7a2cf273   10 months ago   693MB
[root@localhost ~]# docker run -itd --name mongo -p 27017:27017 mongo --auth
1502dc5dda542495981d04c7efefd304ff63cadcc083c7326c8751e9a9e8be57
[root@localhost ~]# docker exec -it mongo mongo admin
MongoDB shell version v5.0.5
connecting to: mongodb://127.0.0.1:27017/admin?compressors=disabled&gssapiServiceName=mongodb
Implicit session: session { "id" : UUID("6449b90e-c703-4373-bf1a-578394740a88") }
MongoDB server version: 5.0.5
================
Warning: the "mongo" shell has been superseded by "mongosh",
which delivers improved usability and compatibility.The "mongo" shell has been deprecated and will be removed in
an upcoming release.
For installation instructions, see
https://docs.mongodb.com/mongodb-shell/install/
================
Welcome to the MongoDB shell.
For interactive help, type "help".
For more comprehensive documentation, see
 https://docs.mongodb.com/
Questions? Try the MongoDB Developer Community Forums
 https://community.mongodb.com
> db.createUser({ user:'admin',pwd:'123456',roles:[ { role:'userAdminAnyDatabase', db: 'admin'},"readWriteAnyDatabase"]});
Successfully added user: {
 "user" : "admin",
 "roles" : [
  {
   "role" : "userAdminAnyDatabase",
   "db" : "admin"
  },
  "readWriteAnyDatabase"
 ]
}
> db.auth('admin', '123456')
1
> 
```

</details>

# docker 安装 **elasticsearch**

## **1. 拉取镜像：**

```bash
sudo docker pull elasticsearch:7.12.0
```

## **2. 创建docker容器挂载目录：**

```bash
sudo mkdir -pv /opt/elasticsearch/config
sudo mkdir -pv /opt/elasticsearch/data
sudo mkdir -pv /opt/elasticsearch/plugins
```

`mkdir` 参数说明：

- -p, --parents     如果存在，则没有错误，根据需要创建父目录
- -v, --verbose     为每个创建的目录打印一条消息

## **3. 配置文件（elasticsearch.yml）：**

```bash
echo "http.host: 0.0.0.0" > /opt/elasticsearch/config/elasticsearch.yml
chmod -R 777 /opt/elasticsearch/
```

参数说明：

- ">"                          如果文件存在，清空文件内容并写入。
                               如果文件不存在，创建新文件并写入。

- -R, --recursive        递归地更改文件和目录

- 777                         可读、可写、可执行权限

## **4. 创建容器：**

```bash
sudo docker run --name elasticsearch -p 9200:9200  -p 9300:9300 \
 -e "discovery.type=single-node" \
 -e ES_JAVA_OPTS="-Xms84m -Xmx512m" \
 -v /opt/elasticsearch/config/elasticsearch.yml:/usr/share/elasticsearch/config/elasticsearch.yml \
 -v /opt/elasticsearch/data:/usr/share/elasticsearch/data \
 -v /opt/elasticsearch/plugins:/usr/share/elasticsearch/plugins \
 -d elasticsearch:7.12.0
```

`docker run`参数说明:

- --name string                    为容器分配一个名称
- -p, --publish list                  向主机发布容器的端口
- -P, --publish-all                    将所有公开的端口发布到随机端口
- -e discovery.type=single-node 单点模式启动
- -e ES_JAVA_OPTS="-Xms84m -Xmx512m"：设置启动占用的内存范围
- -v, --volume list                    绑定挂载卷
- -d, --detach                         在后台运行容器并打印容器ID

## **5. 查看启动详情：**

```bash
docker ps  查看是否启动
docker logs elasticsearch  启动日志查询
docker restart elasticsearch   重启
docker exec -it elasticsearch bash 进入
```

## **6. 安装 elasticsearch-ik 分词器**

==elasticsearch-ik 分词器版本和 elasticsearch 版本必须一致==

```sh
wget https://github.com/medcl/elasticsearch-analysis-ik/releases/download/v7.12.0/elasticsearch-analysis-ik-7.12.0.zip
mkdir -pv /opt/elasticsearch/plugins/ik/
unzip elasticsearch-analysis-ik-7.12.0.zip # 解压到ik目录中
docker restart elasticsearch
```

## **操作明细**

<details>
<summary>点击查看代码</summary>

```bash
[root@localhost ~]# docker pull elasticsearch:7.12.0
7.12.0: Pulling from library/elasticsearch
7a0437f04f83: Pull complete 
2b674c951ca3: Pull complete 
06baeb69f25f: Pull complete 
eeff01d19ce5: Pull complete 
a994306398ca: Pull complete 
2c002d76c1f6: Pull complete 
6286f2196f9b: Pull complete 
Digest: sha256:383e9fb572f3ca2fdef5ba2edb0dae2c467736af96aba2c193722aa0c08ca7ec
Status: Downloaded newer image for elasticsearch:7.12.0
docker.io/library/elasticsearch:7.12.0
[root@localhost ~]# docker images
REPOSITORY      TAG       IMAGE ID       CREATED         SIZE
elasticsearch   7.12.0    9337ed510a0c   18 months ago   830MB
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/config
mkdir: 已创建目录 "/opt/elasticsearch"
mkdir: 已创建目录 "/opt/elasticsearch/config"
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/data
mkdir: 已创建目录 "/opt/elasticsearch/data"
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/plugins
mkdir: 已创建目录 "/opt/elasticsearch/plugins"
[root@localhost config]# echo "http.host: 0.0.0.0" > /opt/elasticsearch/config/elasticsearch.yml
[root@localhost config]# sudo docker run --name elasticsearch -p 9200:9200  -p 9300:9300 \
>  -e "discovery.type=single-node" \
>  -e ES_JAVA_OPTS="-Xms84m -Xmx512m" \
>  -v /opt/elasticsearch/config/elasticsearch.yml:/usr/share/elasticsearch/config/elasticsearch.yml \
>  -v /opt/elasticsearch/data:/usr/share/elasticsearch/data \
>  -v /opt/elasticsearch/plugins:/usr/share/elasticsearch/plugins \
>  -d elasticsearch:7.12.0
8f1930bde13101b5f0412d2e31c7ebc9114c80d95b36da4ead466262574642af
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE                  COMMAND                  CREATED          STATUS             PORTS                                                                                  NAMES
8f1930bde131   elasticsearch:7.12.0   "/bin/tini -- /usr/l…"   13 minutes ago   Up 5 minutes       0.0.0.0:9200->9200/tcp, :::9200->9200/tcp, 0.0.0.0:9300->9300/tcp, :::9300->9300/tcp   elasticsearch

[root@localhost ~]# curl "http://127.0.0.1:9200"
{
  "name" : "8f1930bde131",
  "cluster_name" : "elasticsearch",
  "cluster_uuid" : "XjL5BIXbRrOY0VR4HfloEQ",
  "version" : {
    "number" : "7.12.0",
    "build_flavor" : "default",
    "build_type" : "docker",
    "build_hash" : "78722783c38caa25a70982b5b042074cde5d3b3a",
    "build_date" : "2021-03-18T06:17:15.410153305Z",
    "build_snapshot" : false,
    "lucene_version" : "8.8.0",
    "minimum_wire_compatibility_version" : "6.8.0",
    "minimum_index_compatibility_version" : "6.0.0-beta1"
  },
  "tagline" : "You Know, for Search"
}
```

</details>

# dockerfile

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221007104737622-1275109783.png)

## 基础知识

1. 每个保留关键字（指令）都是必须大写字母
2. 执行从上到下顺序执行
3. "#" 表示注释
4. 每个指令都会创建提交一个新的镜像层，并提交！

## dockerfile 指令介绍

```sh
FROM                 # 基础镜像，一切从这里开始构建
MAINTAINER      # 镜像是谁写的， 姓名+邮箱
RUN                    # 镜像构建的时候需要运行的命令
ADD                    # 步骤， tomcat镜像， 这个tomcat压缩包！添加内容
WORKDIR           # 镜像的工作目录
VOLUME             # 挂载的目录
EXPOSE              # 保留端口配置
CMD                   # 指定这个容器启动的时候要运行的命令，只有最后一个会生效可被替代
ENTRYPOINT      # 指定这个容器启动的时候要运行的命令， 可以追加命令
ONBUILD            # 当构建一个被继承DockerFile 这个时候就会运行 ONBUILD 的指令，触发指令
COPY                  # 类似ADD, 将我们文件拷贝到镜像中
ENV                    # 构建的时候设置环境变量！
```

## 创建一个自己的 centos

### 1. 编写 dockerfile

```dockerfile
FROM centos
MAINTAINER xiaofan<11111111@qq.com>
 
ENV MYPATH /usr/local
WORKDIR $MYPATH     # 镜像的工作目录
 
RUN yum -y install vim
RUN yum -y install net-tools
 
EXPOSE 80
 
CMD echo $MYPATH
CMD echo "---end---"
CMD /bin/bash
```

### 2. 通过这个文件构建镜像

```sh
# 命令 docker build -f dockerfile文件路径 -t 镜像名:[tag] .
[root@ dockerfile]# docker build -f mydockerfile-centos -t mycentos:0.1 .
Successfully built d2d9f0ea8cb2
Successfully tagged mycentos:0.1
```

## dockerfile 制作 tomcat 镜像

### 1. dockerfile 脚本编写

```dockerfile
FROM centos
MAINTAINER xiaofan< 111111@qq.com>
 
COPY readme.txt /usr/local/readme.txt
 
ADD jdk-8u73-linux-x64.tar.gz /usr/local/
ADD apache-tomcat-9.0.37.tar.gz /usr/local/
 
RUN yum -y install vim
 
ENV MYPATH /usr/local
WORKDIR $MYPATH
 
ENV JAVA_HOME /usr/local/jdk1.8.0_73
ENV CLASSPATH $JAVA_HOME/lib/dt.jar:$JAVA_HOME/lib/tools.jar
ENV CATALINA_HOME /usr/local/apache-tomcat-9.0.37
ENV CATALINA_BASH /usr/local/apache-tomcat-9.0.37
ENV PATH $PATH:$JAVA_HOME/bin:$CATALINA_HOME/lib:$CATALINA_HOME/bin
 
EXPOSE 8080
 
CMD /usr/local/apache-tomcat-9.0.37/bin/startup.sh && tail -F /usr/local/apache-tomcat-9.0.37/bin/logs/catalina.out
```

### 2. 构建镜像

```sh
# docker build -t diytomcat .   # "." 注意
```

### 3. 启动镜像

```sh
#  docker run -d -p 3344:8080 --name xiaofantomcat1 -v /home/xiaofan/build/tomcat/test:/usr/local/apache-tomcat-9.0.37/webapps/test -v /home/xiaofan/build/tomcat/tomcatlogs/:/usr/local/apache-tomcat-9.0.37/logs diytomcat
```

### 4. web.xml

```sh

<?xml version="1.0" encoding="UTF-8"?>
<web-app version="2.4" 
    xmlns="http://java.sun.com/xml/ns/j2ee" 
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://java.sun.com/xml/ns/j2ee 
        http://java.sun.com/xml/ns/j2ee/web-app_2_4.xsd">
        
</web-app>
```

### 5. index.jsp

```sh
<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>hello. xiaofan</title>
</head>
<body>
Hello World!<br/>
<%
System.out.println("-----my test web logs------");
%>
</body>
</html>
```

5. curl 访问

```sh
curl "http://127.0.0.1:4443/test/"
```

# **docker0**

参考文档：<https://mp.weixin.qq.com/s/nnV7tsn5dd1mtjB0GObLzA>

## **1. 本机网络**

```sh
[root@localhost ~]# ip addr
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: ens33: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP group default qlen 1000
    link/ether 00:0c:29:20:74:87 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.102/24 brd 192.168.1.255 scope global noprefixroute dynamic ens33
       valid_lft 4413sec preferred_lft 4413sec
    inet6 fe80::686a:cd80:2b45:7881/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
3: docker0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 02:42:2e:6d:69:37 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0
       valid_lft forever preferred_lft forever
    inet6 fe80::42:2eff:fe6d:6937/64 scope link 
       valid_lft forever preferred_lft forever
5: vethe91eda2@if4: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master docker0 state UP group default 
    link/ether 0e:ff:08:fa:d4:19 brd ff:ff:ff:ff:ff:ff link-netnsid 0
    inet6 fe80::cff:8ff:fefa:d419/64 scope link 
       valid_lft forever preferred_lft forever
[root@localhost ~]# 
 网络说明：
 1: lo: 本地回环地址
 2: ens33: 本机内网地址
 3: docker0: docker0 地址
```

## **2. 内网 <-> 容器网络**

```sh
# 问题： docker是如何处理容器网络访问的？
 
# [root@iZ2zeg4ytp0whqtmxbsqiiZ ~]# docker run -d -P --name tomcat01 tomcat
 
# 查看容器内部的网络地址 ip addr
[root@iZ2zeg4ytp0whqtmxbsqiiZ ~]# docker exec -it tomcat01 ip addr， 发现容器启动的时候得到一个eth0@if115 ip地址，docker分配的！
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
114: eth0@if115: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 02:42:ac:11:00:02 brd ff:ff:ff:ff:ff:ff link-netnsid 0
    inet 172.17.0.2/16 brd 172.17.255.255 scope global eth0
       valid_lft forever preferred_lft forever
 
# 思考： linux 能不能ping通容器？
[root@iZ2zeg4ytp0whqtmxbsqiiZ ~]# ping 172.17.0.2
PING 172.17.0.2 (172.17.0.2) 56(84) bytes of data.
64 bytes from 172.17.0.2: icmp_seq=1 ttl=64 time=0.077 ms
64 bytes from 172.17.0.2: icmp_seq=2 ttl=64 time=0.069 ms
64 bytes from 172.17.0.2: icmp_seq=3 ttl=64 time=0.075 ms
 
# linux 可以 ping 通docker容器内部！
```

原理：

1. 我们没启动一个docker容器， docker就会给docker容器分配一个ip， 我们只要安装了docker，就会有一个网卡 docker0桥接模式，使用的技术是veth-pair技术！
2. 每启动一个容器测试， 就会又多了一对网卡
3. 两个容器之间互通

## **3. docker 网络**

```sh
[root@localhost ~]# docker network ls
NETWORK ID     NAME      DRIVER    SCOPE
36015d86d5ac   bridge    bridge    local
ea337009fb83   host      host      local
e4d1ce606183   none      null      local
[root@localhost ~]# 
[root@localhost ~]# 
[root@localhost ~]# docker network inspect 36015d86d5ac
[
    {
        "Name": "bridge",
        "Id": "36015d86d5ac0bd16d51c737ec0d96d965100c02dd74ebbd70eadebe23608724",
        "Created": "2022-10-06T12:07:38.323750128+08:00",
        "Scope": "local",
        "Driver": "bridge",
        "EnableIPv6": false,
        "IPAM": {
            "Driver": "default",
            "Options": null,
            "Config": [
                {
                    "Subnet": "172.17.0.0/16", # 16 255*255
                    "Gateway": "172.17.0.1" # docker0
                }
            ]
        },
        "Internal": false,
        "Attachable": false,
        "Ingress": false,
        "ConfigFrom": {
            "Network": ""
        },
        "ConfigOnly": false,
        "Containers": {
            "720e35066bff12aa459c1e8859d51a0768e3a980951bef7e5d0d7648e2568823": {
                "Name": "redis-test",
                "EndpointID": "4c3e617ddba0b7fa031d739ba51225be92359e2339c43ed69a72e818a7ca6f50",
                "MacAddress": "02:42:ac:11:00:02",
                "IPv4Address": "172.17.0.2/16",
                "IPv6Address": ""
            }
        },
        "Options": {
            "com.docker.network.bridge.default_bridge": "true",
            "com.docker.network.bridge.enable_icc": "true",
            "com.docker.network.bridge.enable_ip_masquerade": "true",
            "com.docker.network.bridge.host_binding_ipv4": "0.0.0.0",
            "com.docker.network.bridge.name": "docker0",
            "com.docker.network.driver.mtu": "1500"
        },
        "Labels": {}
    }
]
[root@localhost ~]# 

```

## **4. --link**

思考一个场景，我们编写了一个微服务，database url =ip； 项目不重启，数据ip换掉了，我们希望可以处理这个问题，可以按名字来进行访问容器

```sh
[root@ ~]# docker exec -it tomcat02 ping tomcat01
ping: tomcat01: Name or service not known
 
# 如何可以解决呢？
# 通过--link既可以解决网络连通问题
[root@ ~]# docker run -d -P  --name tomcat03 --link tomcat02 tomcat
3a2bcaba804c5980d94d168457c436fbd139820be2ee77246888f1744e6bb473
[root@ ~]# docker ps
CONTAINER ID        IMAGE               COMMAND             CREATED             STATUS              PORTS                     NAMES
3a2bcaba804c        tomcat              "catalina.sh run"   4 seconds ago       Up 3 seconds        0.0.0.0:32772->8080/tcp   tomcat03
f22ed47ed1be        tomcat              "catalina.sh run"   57 minutes ago      Up 57 minutes       0.0.0.0:32771->8080/tcp   tomcat02
9d97f93401a0        tomcat              "catalina.sh run"   About an hour ago   Up About an hour    0.0.0.0:32770->8080/tcp   tomcat01
[root@ ~]# docker exec -it tomcat03 ping tomcat02
PING tomcat02 (172.17.0.3) 56(84) bytes of data.
64 bytes from tomcat02 (172.17.0.3): icmp_seq=1 ttl=64 time=0.129 ms
64 bytes from tomcat02 (172.17.0.3): icmp_seq=2 ttl=64 time=0.100 ms
64 bytes from tomcat02 (172.17.0.3): icmp_seq=3 ttl=64 time=0.110 ms
64 bytes from tomcat02 (172.17.0.3): icmp_seq=4 ttl=64 time=0.107 ms
 
# 反向可以ping通吗？
[root@ ~]# docker exec -it tomcat02 ping tomcat03
ping: tomcat03: Name or service not known
 
```

## **5. 查看tomcat容器互通**

```sh
[root@ ~]# docker exec -it tomcat03 cat /etc/hosts
127.0.0.1   localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
ff00::0 ip6-mcastprefix
ff02::1 ip6-allnodes
ff02::2 ip6-allrouters
172.17.0.3  tomcat02 f22ed47ed1be
172.17.0.4  3a2bcaba804c
```

--link 就是我们在hosts配置中增加了一个172.17.0.3 tomcat02 f22ed47ed1be

## **6. docker自定义网络**

```sh
[root@localhost ~]# docker network ls # 查看 docker 网络
NETWORK ID     NAME      DRIVER    SCOPE
36015d86d5ac   bridge    bridge    local
ea337009fb83   host      host      local
e4d1ce606183   none      null      local
[root@localhost ~]# 
网络模式参数说明：
 bridge： 桥接模式，桥接 docker 默认，自己创建的也是用brdge模式
 none： 不配置网络
 host： 和宿主机共享网络
 container：容器网络连通！（用的少， 局限很大）

# 我们直接启动的命令默认有一个 --net bridge，而这个就是我们的docker0
docker run -d -P --name tomcat01 tomcat
docker run -d -P --name tomcat01 --net bridge tomcat
 
[root@ ~]# docker network create --driver bridge --subnet 192.168.0.0/16 --gateway 192.168.0.1 mynet
26a5afdf4805d7ee0a660b82244929a4226470d99a179355558dca35a2b983ec
[root@ ~]# docker network ls
NETWORK ID          NAME                DRIVER              SCOPE
30d601788862        bridge              bridge              local
226019b14d91        host                host                local
26a5afdf4805        mynet               bridge              local
7496c014f74b        none                null                local
```

参数说明：
 docker0特点，默认，容器名不能访问， --link可以打通连接！我们可以自定义一个网络！

- --driver bridge
- --subnet 192.168.0.0/16 可以支持255*255个网络 192.168.0.2 ~ 192.168.255.254
- --gateway 192.168.0.1

==在自己创建的网络里面启动两个容器==

```sh
[root@ ~]# docker run -d -P --name tomcat-net-01 --net mynet tomcat
0e85ebe6279fd23379d39b27b5f47c1e18f23ba7838637802973bf6449e22f5c
[root@ ~]# docker run -d -P --name tomcat-net-02 --net mynet tomcat
c6e462809ccdcebb51a4078b1ac8fdec33f1112e9e416406b606d0c9fb6f21b5
[root@ ~]# docker network inspect mynet # 查看自己创建的网络
[
    {
        "Name": "mynet",
        "Id": "26a5afdf4805d7ee0a660b82244929a4226470d99a179355558dca35a2b983ec",
        "Created": "2020-08-14T11:12:40.553433163+08:00",
        "Scope": "local",
        "Driver": "bridge",
        "EnableIPv6": false,
        "IPAM": {
            "Driver": "default",
            "Options": {},
            "Config": [
                {
                    "Subnet": "192.168.0.0/16",
                    "Gateway": "192.168.0.1"
                }
            ]
        },
        "Internal": false,
        "Attachable": false,
        "Ingress": false,
        "ConfigFrom": {
            "Network": ""
        },
        "ConfigOnly": false,
        "Containers": {
            "0e85ebe6279fd23379d39b27b5f47c1e18f23ba7838637802973bf6449e22f5c": {
                "Name": "tomcat-net-01",
                "EndpointID": "576ce5c0f5860a5aab5e487a805da9d72f41a409c460f983c0bd341dd75d83ac",
                "MacAddress": "02:42:c0:a8:00:02",
                "IPv4Address": "192.168.0.2/16",
                "IPv6Address": ""
            },
            "c6e462809ccdcebb51a4078b1ac8fdec33f1112e9e416406b606d0c9fb6f21b5": {
                "Name": "tomcat-net-02",
                "EndpointID": "81ecbc4fe26e49855fe374f2d7c00d517b11107cc91a174d383ff6be37d25a30",
                "MacAddress": "02:42:c0:a8:00:03",
                "IPv4Address": "192.168.0.3/16",
                "IPv6Address": ""
            }
        },
        "Options": {},
        "Labels": {}
    }
]

# 再次 ping 连接
[root@ ~]# docker exec -it tomcat-net-01 ping 192.168.0.3
PING 192.168.0.3 (192.168.0.3) 56(84) bytes of data.
64 bytes from 192.168.0.3: icmp_seq=1 ttl=64 time=0.113 ms
64 bytes from 192.168.0.3: icmp_seq=2 ttl=64 time=0.093 ms
^C
--- 192.168.0.3 ping statistics ---
2 packets transmitted, 2 received, 0% packet loss, time 999ms
rtt min/avg/max/mdev = 0.093/0.103/0.113/0.010 ms
# 现在不使用 --link也可以ping名字了！
[root@ ~]# docker exec -it tomcat-net-01 ping tomcat-net-02
PING tomcat-net-02 (192.168.0.3) 56(84) bytes of data.
64 bytes from tomcat-net-02.mynet (192.168.0.3): icmp_seq=1 ttl=64 time=0.068 ms
64 bytes from tomcat-net-02.mynet (192.168.0.3): icmp_seq=2 ttl=64 time=0.096 ms
64 bytes from tomcat-net-02.mynet (192.168.0.3): icmp_seq=3 ttl=64 time=0.094 ms

```

不同的集群使用不同的网络，保证集群时安全和健康的

打通tomcat01 和mynet
![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221007114107623-393892729.png)

```sh
[root@ ~]# docker network connect  mynet tomcat01

# 连通之后就是讲tomcat01 放到了mynet网路下
# 一个容器两个ip地址：
# 公网ip，私网ip


# 连通测试ok
[root@ ~]# docker exec -it tomcat01 ping tomcat-net-01
PING tomcat-net-01 (192.168.0.2) 56(84) bytes of data.
64 bytes from tomcat-net-01.mynet (192.168.0.2): icmp_seq=1 ttl=64 time=0.100 ms
64 bytes from tomcat-net-01.mynet (192.168.0.2): icmp_seq=2 ttl=64 time=0.085 ms
^C
--- tomcat-net-01 ping statistics ---
2 packets transmitted, 2 received, 0% packet loss, time 1000ms
rtt min/avg/max/mdev = 0.085/0.092/0.100/0.012 ms
# 依旧无法连通，没有connect
[root@ ~]# docker exec -it tomcat02 ping tomcat-net-01
ping: tomcat-net-01: Name or service not known
 
```

# **docker 部署 redis 网络集群**

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221007114612071-2043523685.png)

```sh
## 1. 创建网卡
docker network create redis --subnet 172.38.0.0/16
 
# 2. 通过脚本创建六个redis配置
for port in $(seq 1 6); \
do \
mkdir -p /mydata/redis/node-${port}/conf
touch /mydata/redis/node-${port}/conf/redis.conf
cat << EOF >/mydata/redis/node-${port}/conf/redis.conf
port 6379
bind 0.0.0.0
cluster-enabled yes
cluster-config-file nodes.conf
cluster-node-timeout 5000
cluster-announce-ip 172.38.0.1${port}
cluster-announce-port 6379
cluster-announce-bus-port 16379
appendonly yes
EOF
done
# 3.0 创建结点1
docker run -p 6371:6379 -p 16371:16379 --name redis-1 \
-v /mydata/redis/node-1/data:/data \
-v /mydata/redis/node-1/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.11 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
 
# 3.1 创建结点2
docker run -p 6372:6379 -p 16372:16379 --name redis-2 \
-v /mydata/redis/node-2/data:/data \
-v /mydata/redis/node-2/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.12 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
# 3.2 创建结点3
docker run -p 6373:6379 -p 16373:16379 --name redis-3 \
-v /mydata/redis/node-3/data:/data \
-v /mydata/redis/node-3/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.13 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
# 3.3 创建结点4
docker run -p 6374:6379 -p 16374:16379 --name redis-4 \
-v /mydata/redis/node-4/data:/data \
-v /mydata/redis/node-4/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.14 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
# 3.4 创建结点5
docker run -p 6375:6379 -p 16375:16379 --name redis-5 \
-v /mydata/redis/node-5/data:/data \
-v /mydata/redis/node-5/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.15 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
# 3.5创建结点6
docker run -p 6376:6379 -p 16376:16379 --name redis-6 \
-v /mydata/redis/node-6/data:/data \
-v /mydata/redis/node-6/conf/redis.conf:/etc/redis/redis.conf \
-d --net redis --ip 172.38.0.16 redis:5.0.9-alpine3.11 redis-server /etc/redis/redis.conf
 
# 4. 创建集群
[root@ ~]# docker exec -it redis-1 /bin/sh
/data # ls
appendonly.aof  nodes.conf
/data # redis-cli --cluster create 172.38.0.11:6379 172.38.0.12:6379 172.38.0.13:6379 172.38.0.14:6379 172.38.0.15:6379 172.38.0.16:6379 --cluster-replicas 1
>>> Performing hash slots allocation on 6 nodes...
Master[0] -> Slots 0 - 5460
Master[1] -> Slots 5461 - 10922
Master[2] -> Slots 10923 - 16383
Adding replica 172.38.0.15:6379 to 172.38.0.11:6379
Adding replica 172.38.0.16:6379 to 172.38.0.12:6379
Adding replica 172.38.0.14:6379 to 172.38.0.13:6379
M: 541b7d237b641ac2ffc94d17c6ab96b18b26a638 172.38.0.11:6379
   slots:[0-5460] (5461 slots) master
M: a89c1f1245b264e4a402a3cf99766bcb6138dbca 172.38.0.12:6379
   slots:[5461-10922] (5462 slots) master
M: 259e804d6df74e67a72e4206d7db691a300c775e 172.38.0.13:6379
   slots:[10923-16383] (5461 slots) master
S: 9b19170eea3ea1b92c58ad18c0b5522633a9e271 172.38.0.14:6379
   replicates 259e804d6df74e67a72e4206d7db691a300c775e
S: 061a9d38f22910aaf0ba1dbd21bf1d8f57bcb7d5 172.38.0.15:6379
   replicates 541b7d237b641ac2ffc94d17c6ab96b18b26a638
S: 7a16b9bbb0615ec95fc978fa62fc054df60536f0 172.38.0.16:6379
   replicates a89c1f1245b264e4a402a3cf99766bcb6138dbca
Can I set the above configuration? (type 'yes' to accept): yes
>>> Nodes configuration updated
>>> Assign a different config epoch to each node
>>> Sending CLUSTER MEET messages to join the cluster
Waiting for the cluster to join
...
>>> Performing Cluster Check (using node 172.38.0.11:6379)
M: 541b7d237b641ac2ffc94d17c6ab96b18b26a638 172.38.0.11:6379
   slots:[0-5460] (5461 slots) master
   1 additional replica(s)
M: a89c1f1245b264e4a402a3cf99766bcb6138dbca 172.38.0.12:6379
   slots:[5461-10922] (5462 slots) master
   1 additional replica(s)
S: 7a16b9bbb0615ec95fc978fa62fc054df60536f0 172.38.0.16:6379
   slots: (0 slots) slave
   replicates a89c1f1245b264e4a402a3cf99766bcb6138dbca
S: 061a9d38f22910aaf0ba1dbd21bf1d8f57bcb7d5 172.38.0.15:6379
   slots: (0 slots) slave
   replicates 541b7d237b641ac2ffc94d17c6ab96b18b26a638
M: 259e804d6df74e67a72e4206d7db691a300c775e 172.38.0.13:6379
   slots:[10923-16383] (5461 slots) master
   1 additional replica(s)
S: 9b19170eea3ea1b92c58ad18c0b5522633a9e271 172.38.0.14:6379
   slots: (0 slots) slave
   replicates 259e804d6df74e67a72e4206d7db691a300c775e
[OK] All nodes agree about slots configuration.
>>> Check for open slots...
>>> Check slots coverage...
[OK] All 16384 slots covered.
 
 
```

# **SpringBoot微服务打包Docker镜像**

## 1. 构建 springboot 项目

## 2. 打包 *.jar

## 3. 编写 dockerfile

```dockerfile
FROM java:8
COPY *.jar /app.jar
CMD ["--server.port=8080"]
EXPOSE 8080
ENTRYPOINT ["java", "-jar", "/app.jar"]
```

## 4. 构建镜像、发布运行、测试

```sh
# 把打好的jar包和Dockerfile上传到linux
[root@ idea]# ll
total 16140
-rw-r--r-- 1 root root 16519871 Aug 14 17:38 demo-0.0.1-SNAPSHOT.jar
-rw-r--r-- 1 root root      122 Aug 14 17:38 dockerfile
 
# 构建镜像，不要忘了最后有一个点
[root@ idea]# docker build -t xiaofan666 .
Sending build context to Docker daemon  16.52MB
Step 1/5 : FROM java:8
8: Pulling from library/java
5040bd298390: Pull complete 
fce5728aad85: Pull complete 
76610ec20bf5: Pull complete 
60170fec2151: Pull complete 
e98f73de8f0d: Pull complete 
11f7af24ed9c: Pull complete 
49e2d6393f32: Pull complete 
bb9cdec9c7f3: Pull complete 
Digest: sha256:c1ff613e8ba25833d2e1940da0940c3824f03f802c449f3d1815a66b7f8c0e9d
Status: Downloaded newer image for java:8
 ---> d23bdf5b1b1b
Step 2/5 : COPY *.jar /app.jar
 ---> d4de8837ebf9
Step 3/5 : CMD ["--server.port=8080"]
 ---> Running in e3abc66303f0
Removing intermediate container e3abc66303f0
 ---> 131bb3917fea
Step 4/5 : EXPOSE 8080
 ---> Running in fa2f25977db7
Removing intermediate container fa2f25977db7
 ---> d98147377951
Step 5/5 : ENTRYPOINT ["java", "-jar", "/app.jar"]
 ---> Running in e1885e23773b
Removing intermediate container e1885e23773b
 ---> afb6b5f28a32
Successfully built afb6b5f28a32
Successfully tagged xiaofan666:latest
 
# 查看镜像
[root@ idea]# docker images
REPOSITORY          TAG                 IMAGE ID            CREATED             SIZE
xiaofan666          latest              afb6b5f28a32        14 seconds ago      660MB
tomcat              latest              2ae23eb477aa        8 days ago          647MB
redis               5.0.9-alpine3.11    3661c84ee9d0        3 months ago        29.8MB
java                8                   d23bdf5b1b1b        3 years ago         643MB
 
# 运行容器
[root@ idea]# docker run -d -P --name xiaofan-springboot-web xiaofan666
fd9a353a80bfd61f6930c16cd92204532bfd734e003f3f9983b5128a27b0375e
# 查看运行起来的容器端口（因为我们启动的时候没有指定）
[root@iZ2zeg4ytp0whqtmxbsqiiZ idea]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                     NAMES
fd9a353a80bf        xiaofan666          "java -jar /app.jar …"   9 seconds ago       Up 8 seconds        0.0.0.0:32779->8080/tcp   xiaofan-springboot-web
# 本地访问1
[root@ idea]# curl localhost:32779
{"timestamp":"2020-08-14T09:42:57.371+00:00","status":404,"error":"Not Found","message":"","path":"/"}
# 本地访问2
[root@ idea]# [root@iZ2zeg4ytp0whqtmxbsqiiZ idea]# curl localhost:32779/hello
hello, xiaofan
# 远程访问（开启阿里云上的安全组哦）
```

# **将我们容器内的目录挂载到linux目录上面（docker 容器数据卷）**

## 1. 方式一：直接使用命令来挂载 -v

```bash
# docker run -it -v 主机目录：容器目录
[root@ home]# docker run -it -v /home/ceshi:/home centos /bin/bash
```

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221007101555313-458292641.png)

总结：容器运行时，挂载目录数据一直同步

---

```bash
# 获取镜像
[root@ home]# docker pull mysql:5.7
# 运行容器， 需要做数据挂载！ # 安装启动mysql，需要配置密码（注意）
# 官方测试， docker run --name some-mysql -e MYSQL_ROOT_PASSWORD=my-secret-pw -d mysql:tag
[root@ home]# docker run -d -p 3344:3306 -v /home/mysql/conf:/etc/mysql/conf.d -v /home/mysql/data:/var/lib/mysql -e MYSQL_ROOT_PASSWORD=123456 --name mysql01 mysql:5.7
9552bf4eb2b69a2ccd344b5ba5965da4d97b19f2e1a78626ac1f2f8d276fc2ba

 # 参数说明：
 -d      # 后台运行
 -p      # 端口隐射
 -v      # 卷挂载
 -e      # 环境配置
 --name  # 容器的名字
# 启动成功之后，我们在本地使用navicat链接测试一下
# navicat链接到服务器的3344 --- 3344 和 容器的3306映射，这个时候我们就可以连接上mysql喽！
 
# 在本地测试创建一个数据库，查看下我们的路径是否ok！
```

## 2. 匿名和具名挂载

```bash
# 匿名挂载
-v 容器内路径
docker run -d -P --name nginx01 -v /etc/nginx nginx     # -P 随机指定端口
 
# 查看所有volume的情况
[root@ ~]# docker volume ls
DRIVER              VOLUME NAME
local               561b81a03506f31d45ada3f9fb7bd8d7c9b5e0f826c877221a17e45d4c80e096
local               36083fb6ca083005094cbd49572a0bffeec6daadfbc5ce772909bb00be760882
 
# 这里发现，这种情况就是匿名挂载，我们在-v 后面只写了容器内的路径，没有写容器外的路径！
 
# 具名挂载
[root@ ~]# docker run -d -P --name nginx02 -v juming-nginx:/etc/nginx nginx
26da1ec7d4994c76e80134d24d82403a254a4e1d84ec65d5f286000105c3da17
[root@ ~]# docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                   NAMES
26da1ec7d499        nginx               "/docker-entrypoint.…"   3 seconds ago       Up 2 seconds        0.0.0.0:32769->80/tcp   nginx02
486de1da03cb        nginx               "/docker-entrypoint.…"   3 minutes ago       Up 3 minutes        0.0.0.0:32768->80/tcp   nginx01
[root@ ~]# docker volume ls
DRIVER              VOLUME NAME
local               561b81a03506f31d45ada3f9fb7bd8d7c9b5e0f826c877221a17e45d4c80e096
local               36083fb6ca083005094cbd49572a0bffeec6daadfbc5ce772909bb00be760882
local               juming-nginx
 
# 通过-v 卷名：容器内的路径
# 查看一下这个卷
# docker volume inspect juming-nginx

[root@ ~]# docker volume inspect juming-nginx
[
  {
      "CreatedAt": "2020-08-12T18:15:21+08:00",
      "Driver": "local",
      "Labels": null,
      "Mountpoint": "/var/lib/docker/volumes/juming-nginx/_data",
      "Name": "juming-nginx",
      "Options": null,
      "Scope": "local"
  }
]
```

```sh
# 通过 -v 容器内容路径 ro rw 改变读写权限
ro  readonly    # 只读
rw  readwrite   # 可读可写
 
docker run -d -P --name nginx02 -v juming-nginx:/etc/nginx:ro nginx
docker run -d -P --name nginx02 -v juming-nginx:/etc/nginx:rw nginx
 
# ro 只要看到ro就说明这个路径只能通过宿主机来操作，容器内容无法操作
```

## 从容器内拷贝文件到主机上

```bash
[root@localhost ~]# docker images
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
redis        latest    7614ae9453d1   9 months ago    113MB
mysql        latest    3218b38490ce   9 months ago    516MB
centos       latest    5d0da3dc9764   12 months ago   231MB
[root@localhost ~]# docker run -it centos /bin/bash
[root@30dde7df25fb /]# cd /root/
[root@30dde7df25fb ~]# ls
anaconda-ks.cfg  anaconda-post.log  original-ks.cfg
[root@30dde7df25fb ~]# > Test.java
[root@30dde7df25fb ~]# ls
Test.java  anaconda-ks.cfg  anaconda-post.log  original-ks.cfg
[root@30dde7df25fb ~]# exit
exit
[root@localhost ~]# docker images
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
redis        latest    7614ae9453d1   9 months ago    113MB
mysql        latest    3218b38490ce   9 months ago    516MB
centos       latest    5d0da3dc9764   12 months ago   231MB
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
[root@localhost ~]# docker ps -qa
30dde7df25fb
[root@localhost ~]# docker cp 30dde7df25fb:/root/Test.java /root/
[root@localhost ~]# ls
anaconda-ks.cfg  dump.rdb  Test.java
```

# **docker swarm**

## 1. 初始化一个节点 `docker swarm init`（docker-1）

```sh
[root@localhost ~]# docker network ls
NETWORK ID     NAME      DRIVER    SCOPE
6090f5e5a6ee   bridge    bridge    local
ea337009fb83   host      host      local
e4d1ce606183   none      null      local
[root@localhost ~]# docker swarm -h
Flag shorthand -h has been deprecated, please use --help

Usage:  docker swarm COMMAND

Manage Swarm

Commands:
  ca          Display and rotate the root CA
  init        Initialize a swarm # 初始化一个集群
  join        Join a swarm as a node and/or manager # 以节点的身份加入一个蜂群并进行管理
  join-token  Manage join tokens
  leave       Leave the swarm
  unlock      Unlock swarm
  unlock-key  Manage the unlock key
  update      Update the swarm

Run 'docker swarm COMMAND --help' for more information on a command.
[root@localhost ~]# docker swarm init -h
Flag shorthand -h has been deprecated, please use --help

Usage:  docker swarm init [OPTIONS]

Initialize a swarm

Options:
      --advertise-addr string                  Advertised address (format: # 发布地址
                                               <ip|interface>[:port])
      --autolock                               Enable manager autolocking (requiring an
                                               unlock key to start a stopped manager)
      --availability string                    Availability of the node
                                               ("active"|"pause"|"drain") (default "active")
      --cert-expiry duration                   Validity period for node certificates
                                               (ns|us|ms|s|m|h) (default 2160h0m0s)
      --data-path-addr string                  Address or interface to use for data
                                               path traffic (format: <ip|interface>)
      --data-path-port uint32                  Port number to use for data path traffic
                                               (1024 - 49151). If no value is set or is
                                               set to 0, the default port (4789) is used.
      --default-addr-pool ipNetSlice           default address pool in CIDR format
                                               (default [])
      --default-addr-pool-mask-length uint32   default address pool subnet mask length
                                               (default 24)
      --dispatcher-heartbeat duration          Dispatcher heartbeat period
                                               (ns|us|ms|s|m|h) (default 5s)
      --external-ca external-ca                Specifications of one or more
                                               certificate signing endpoints
      --force-new-cluster                      Force create a new cluster from current state
      --listen-addr node-addr                  Listen address (format:
                                               <ip|interface>[:port]) (default 0.0.0.0:2377)
      --max-snapshots uint                     Number of additional Raft snapshots to retain
      --snapshot-interval uint                 Number of log entries between Raft
                                               snapshots (default 10000)
      --task-history-limit int                 Task history retention limit (default 5)
[root@localhost ~]# ip a
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: ens33: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP group default qlen 1000
    link/ether 00:0c:29:20:74:87 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.102/24 brd 192.168.1.255 scope global noprefixroute dynamic ens33
       valid_lft 6123sec preferred_lft 6123sec
    inet6 fe80::686a:cd80:2b45:7881/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
3: docker0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 02:42:a7:c7:42:08 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 brd 172.17.255.255 scope global docker0
       valid_lft forever preferred_lft forever
    inet6 fe80::42:a7ff:fec7:4208/64 scope link 
       valid_lft forever preferred_lft forever
5: veth5ea21b5@if4: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master docker0 state UP group default 
    link/ether c6:73:7a:3c:f6:6d brd ff:ff:ff:ff:ff:ff link-netnsid 0
    inet6 fe80::c473:7aff:fe3c:f66d/64 scope link 
       valid_lft forever preferred_lft forever
[root@localhost ~]# docker swarm init --advertise-addr 192.168.1.102
Swarm initialized: current node (lxry278x1aixiyahh6lu6zfxx) is now a manager.

To add a worker to this swarm, run the following command:
    # 使用以下指令在其他节点中执行，加入到当前集群中
    docker swarm join --token SWMTKN-1-3tubipd5uic1iklj44smzgc3zaqtizrp7p1llvac4p1yxzf67b-afuzei64x988refyd7gmcxdh9 192.168.1.102:2377

To add a manager to this swarm, run 'docker swarm join-token manager' and follow the instructions.

[root@localhost ~]# 
```

## 2. 生成 docker 加入集群命令 `docker swarm join-token worker`(docker-1)

```sh
[root@localhost ~]# docker swarm join-token worker # 生成 docker 加入集群命令
To add a worker to this swarm, run the following command:

    docker swarm join --token SWMTKN-1-3tubipd5uic1iklj44smzgc3zaqtizrp7p1llvac4p1yxzf67b-afuzei64x988refyd7gmcxdh9 192.168.1.102:2377

[root@localhost ~]# 
```

## 3. 生成一个设置主节点命令`docker swarm join-token manager`(docker-1)

```
[root@localhost ~]# docker swarm join-token manager
To add a manager to this swarm, run the following command:

    docker swarm join --token SWMTKN-1-3tubipd5uic1iklj44smzgc3zaqtizrp7p1llvac4p1yxzf67b-9k0nke0pk02o3ing6nbzingh4 192.168.1.102:2377

[root@localhost ~]# 

```

## 4. 加入一个节点 `docker swarm join`（docker-2）

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008001902117-517100503.png)
![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008002240233-2021339599.png)
![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008003140506-519389294.png)

## **参考文档：**

[Docker Swarm 介绍和工作原理]<https://blog.csdn.net/qq1010267837/article/details/125003810>
