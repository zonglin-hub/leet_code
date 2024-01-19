运行容器开启容器自启
docker update --restart=always 356c575a09aa

**docker 安装 elasticsearch**

<https://blog.csdn.net/qq_44732146/article/details/120744829>

**docker 安装 redis**

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

**docker 安装 mysql**

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

**docker 安装 mongo**

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
