## 1. Linux 导出 MySQL 数据库

1.1. 导出表结构

```sh
mysqldump -u fort -p -d fort > fort.sql
```

1.2. 导出数据和表结构

```sh
# mysql 路径 -u 用户名 -p[密码不要写在明显处] 数据库名 > 数据库名.sql
root@FORT:~# /usr/local/mysql/bin/ mysqldump -u fort -p fort > fort.sql
Enter password: 
```

## 2. Linux 导入 MySQL 数据库

2.1. 创建空数据库

```sh
mysql> create database IF NOT EXISTS fort;
```

2.2. 选择数据库

```sh
mysql> use fort;
```

2.3. 设置utf8

```sh
mysql> set names utf8;
```

2.4. 导入数据

```sh
mysql> source /usr/local/fort.sql;
```
