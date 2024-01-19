参考文档：
[Docker设置容器开机自启](https://blog.csdn.net/qq_41054313/article/details/104297746)

---

## 1、设置 Docker 开机自启

`systemctl enable docker`

## 2、设置 Docker 容器开机自启

### 2.1、方案一：创建 Docker 容器时设置开机自启

在使用 <u>docker run</u> 启动容器时，使用 <u>--restart</u> 参数来设置：
`docker run --restart=always --name imagesName`

### 2.2、方案二：修改已创建的 Docker 容器开机自启

如果创建时未指定 <u>--restart=always</u>，可通过 update 命令
`docker update --restart=always imagesName`

参数说明

`--restart` 具体参数值详细信息：

* `no` - 容器退出时，不重启容器
* `on-failure` - 只有在非0状态退出时才从新启动容器
* `always` - 无论退出状态是如何，都重启容器
* `on - failure` - 指定 Docker 将尝试重新启动容器的最大次数。默认情况下，Docker将尝试永远重新启动容器

  `docker run --restart=on-failure:10 imagesName`
