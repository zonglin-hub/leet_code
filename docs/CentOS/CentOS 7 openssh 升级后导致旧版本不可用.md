```sh
# 把旧版本的加密算法让它重新生效
echo 'PubkeyAcceptedKeyTypes ssh-ed25519,ssh-rsa,rsa-sha2-256,rsa-sha2-512' >> /etc/ssh/sshd_config

# 重启 ssh服务
/etc/init.d/sshd restart
```
