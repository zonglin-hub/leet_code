```yaml
version: '3.5'
services:
  rabbitmq:
    restart: always
    image: rabbitmq:management
    container_name: rabbitmq
    hostname: rabbit
    ports:
      - 5672:5672
      - 15672:15672
    environment:
      TZ: Asia/Shanghai
      RABBITMQ_DEFAULT_USER: root
      RABBITMQ_DEFAULT_PASS: root
    volumes:
      - ./data:/var/lib/rabbitmq
      - ./conf:/etc/rabbitmq
```

```yaml
docker-compose up -d
```



```shell
curl localhost:5672 --output run.log |cat run.log

```

![image](https://img2023.cnblogs.com/blog/2402369/202303/2402369-20230309180240206-743458410.png)


```shell
docker exec -it rabbitmq /bin/bash
```

```shell
/opt/rabbitmq/sbin/rabbitmq-plugins enable rabbitmq_management	# 进入rabbitmq的可执行命令目录 图形化界面默认是关闭的，这里需要开启
```