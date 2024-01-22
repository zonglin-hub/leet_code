# Docker 部署 思源笔记

## 参考文档

- [思源笔记群辉 docker 安装简略版](https://ld246.com/article/1627285006996)
- [思源笔记docker部署 - 建站教程](https://jiuaidu.com/jianzhan/936357/)

---

```bash
# 拉取镜像
docker pull b3log/siyuan

# 运行
docker run --name siyuan -it -d --restart=always \
    -v /usr/local/software/siyuan/data/SiYuan:/root/Documents/SiYuan \
    -p 6806:6806 b3log/siyuan
```

访问路径

- ​`http://192.168.0.104:6806`​
