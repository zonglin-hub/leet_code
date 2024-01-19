配置Docker的服务器开启远程客户端访问：

`sudo vim /etc/systemd/system/multi-user.target.wants/docker.service`

打开后，添加高亮部分：

![image](https://img2023.cnblogs.com/blog/2402369/202303/2402369-20230309232513151-462240788.png)

```livecodeserver
[Service]
Type=notify
# the default is not to use systemd for cgroups because the delegate issues still
# exists and systemd currently does not support the cgroup feature set required
# for containers run by docker
ExecStart=/usr/bin/dockerd -H fd:// -H tcp://0.0.0.0 --containerd=/run/containerd/containerd.sock
ExecReload=/bin/kill -s HUP $MAINPID
TimeoutSec=0
RestartSec=2
Restart=always
```

修改完成后，重启Docker服务，如果是云服务器，记得开启 2375 TCP连接端口：

```sh
sudo systemctl daemon-reload
sudo systemctl restart docker.service
```

现在接着在IDEA中进行配置：

![image](https://img2023.cnblogs.com/blog/2402369/202303/2402369-20230309232620574-518480260.png)

在引擎 API URL 处填写我们 Docker 服务器的IP地址：

```shell
[root@localhost local]# ss -lnp | grep 2375
tcp    LISTEN     0      128      :::2375                 :::*                   users:(("dockerd",pid=6690,fd=3))
```

```sh
tcp://IP:2375
```

显示连接成功后，表示配置正确，点击保存即可。
