# docker-compose 搭建 WebSSH

参考文档：

- [Docker-搭建一个网页版的WebSSH工具](https://www.ywsj.cf/archives/docker--da-jian-yi-ge-wang-ye-ban-de-webssh-gong-ju---zhi-chi-wen-jian-shang-chuan-xia-zai)
- [简易在线终端和sftp工具](https://github.com/Jrohy/webssh)
- [Docker-搭建一个网页版的WebSSH工具](https://www.bilibili.com/video/BV1Zt4y1N7ub/?spm_id_from=333.788&vd_source=9bfc54d2ed901f1eab04708cc346c2f5)

---

创建 `docker-compose.yml`文件

```yaml
version: "3.5"

services:
  lcloud-webssh: #服务名，可以自定义
    image: jrohy/webssh #镜像名不要改
    container_name: lcloud-webssh #容器名，可以自定义
    restart: always #开启开机自启动
    environment:
            - PUID=0 # 稍后在终端输入id可以查看当前用户的id
            - PGID=0 # 同上
            - TZ=Asia/Shanghai #时区，可以自定义
    ports:
      - 2222:5032 # 2222可以改成任意vps上未使用过的端口，5032不要改
```

部署运行

```shell
docker-compose up -d
```

登录webSSH页面 [http://127.0.0.1:2222/](http://127.0.0.1:2222/)
