# Dcoekr 制作 jdk-17 镜像

**参考文档：**

- [jdk17.0.4.1镜像](https://www.cnblogs.com/gkmin/p/16620528.html)
- [Docker之dockerfile制作jdk镜像](https://www.cnblogs.com/huangting/p/11966450.html)

---

**'JDK 官网下载'**

**编写 Dockerfile**

```sh
FROM centos:7
MAINTAINER xxx "xxx@qq.com"
WORKDIR /javaxh_docker/jdk
ADD jdk-17_linux-x64_bin.tar.gz /javaxh_docker/jdk/
ENV JAVA_HOME=/javaxh_docker/jdk/jdk-17.0.6
ENV CLASSPATH=.:$JAVA_HOME/lib/dt.jar:$JAVA_HOME/lib/tools.jar
ENV PATH=$JAVA_HOME/bin:$PATH
CMD ["java","-version"]
```

**构建镜像**

```sh
docker build -t jdk17:v1.0 .  
```

**运行容器**

```sh
docker run -it jdk17:v1.0 /bin/bash
```

**参考案例：**

```dockerfile
FROM anapsix/alpine-java:8_server-jre_unlimited
MAINTAINER opgames(opgames.cn@gmail.com)
RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN mkdir -p /opt/projects/mall4j
WORKDIR /opt/projects/mall4j
ADD ./yami-shop-admin/target/yami-shop-admin-0.0.1-SNAPSHOT.jar ./
EXPOSE 8085
CMD java -jar -Xms512m -Xmx512m -Xss256k -XX:SurvivorRatio=8 -XX:+UseConcMarkSweepGC -Dspring.profiles.active=docker,quartz yami-shop-admin-0.0.1-SNAPSHOT.jar
```
