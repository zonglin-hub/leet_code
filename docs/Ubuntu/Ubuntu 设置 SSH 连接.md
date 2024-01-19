1. ubuntu 查看 ip 信息

	```
	$ ifconfig
	```

2. 若提示找不到命令，则需安装 net-tools

	```
	$ sudo apt-get install net-tools
	```

3. 安装openssl

	```
	$ sudo apt-get install update
	$ sudo apt-get install openssh-server
	```

4. 启动 ssh 服务

	```
	$ sudo service ssh start  或者  sudo /etc/init.d/ssh start
	```