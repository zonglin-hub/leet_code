**参考博客**

- <https://www.cnblogs.com/zhi-leaf/p/5983550.html>
- <https://www.cnblogs.com/52lxl-top/p/9863483.html>

---

- 检查是否安装了ftp

  ```sh
  rpm -qa | grep vsftpd
  ```

- 安装 vsftpd

  ```sh
  yum -y install vsftpd
  chkconfig vsftpd on     # 设置开机自启
  ```

- 修改vsftpd配置 `vim /etc/vsftpd/vsftpd.conf`

  ![image](https://img2020.cnblogs.com/blog/2402369/202112/2402369-20211229173018533-949094984.png)

- 重启 vsftpd服务

  sudo systemctl status vsftpd

`service vsftpd restart`

- 为ftp创建用户(问题！)

  ftp用户名:testftp，密码testftp,并将用户绑定到 /var/ftp/testftp

  useradd  -d /var/ftp/testftp testftp

  设置密码:

  passwd testftp

  输入密码，Linux下输入密码不显示

- 开启防火墙21端口

  iptables -I INPUT -m state --state NEW -m tcp -p tcp --dport 21 -j ACCEPT

  保存：service iptables save

  重启：service iptables restart
  ![image](https://img2020.cnblogs.com/blog/2402369/202112/2402369-20211229174940630-534296589.png)

- 在浏览器测试是否成功

  ![image](https://img2020.cnblogs.com/blog/2402369/202112/2402369-20211229174654377-2035879970.png)

- 权限设置

  如果建新文件夹时出现 550 Create directory operation failed. （550报错）时，或者切换目录出错，应该是没有权限导致的！

  vi /etc/selinux/config

  打开配置将 SELINUX的值设为disabled

重启ftp服务

- 指定ftp家目录

  修改ftp的根目录只要修改/etc/vsftpd/vsftpd.conf文件即可

  vi /etc/vsftpd/vsftpd.conf

  加入下面三行

  local_root=/var/www/html
  chroot_local_user=YES
  anon_root=/var/www/html
  local_root 针对系统用户；anon_root 针对匿名用户

  chmod 755 /var/ftp/testftp

  然后重启ftp服务就可以了

  这时任何一个用户访问都会指定到  /var/ftp/testftp  下   即：ftp的根目录设置成了 /

  FTP配置到此结束！！！

  service vsftpd start 启动ftp命令

  service vsftpd stop 停止ftp命令

  service vsftpd restart 重启ftp命令

## 报错

![image](https://img2020.cnblogs.com/blog/2402369/202201/2402369-20220103143959330-617575353.png)
<https://blog.csdn.net/china_skag/article/details/7278923>
![image](https://img2020.cnblogs.com/blog/2402369/202201/2402369-20220103144413977-2098771522.png)
<https://blog.csdn.net/jiangshuanshuan/article/details/84846611>
![image](https://img2020.cnblogs.com/blog/2402369/202201/2402369-20220103145254740-1036715787.png)

参考博客
<https://www.linuxprobe.com/linux-ftpserver.html>
