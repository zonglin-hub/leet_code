# Docker 部署 RabbitMQ

- [docker-compose安装RabbitMQ_rabbitmq](https://blog.csdn.net/qq_41865652/article/details/123263529)
- [Docker系列之RabbitMQ安装部署教程](https://cloud.tencent.com/developer/article/1612598)

---

## 拉取镜像

```sh
docker pull rabbitmq:management
```

## 运行容器

```sh
docker run --restart=always -d -p 15672:15672 -p 5672:5672 \
    -e RABBITMQ_DEFAULT_USER=admin -e RABBITMQ_DEFAULT_PASS=admin \
    --name lcloud-rabbitmq --hostname=rabbitmqhostone  rabbitmq:management
```

## 访问路径

[RabbitMQ Management](http://192.168.1.102:15672/#/)
