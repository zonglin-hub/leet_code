<details>
<summary>elasticsearch 常用指令</summary>

```bash
root@LAS:~# curl 'http://127.0.0.1:9200/_cat/nodes' # 查询所有节点
192.168.31.127 49 61 0 2.16 2.11 2.03 dilmrt * node-1
root@LAS:~# curl -u elastic:Z37ufZpU -k 'http://127.0.0.1:9200/_cluster/health?pretty' # 查询集群状态
{
  "cluster_name" : "bigdata",
  "status" : "green",
  "timed_out" : false,
  "number_of_nodes" : 1,
  "number_of_data_nodes" : 1,
  "active_primary_shards" : 13,
  "active_shards" : 13,
  "relocating_shards" : 0,
  "initializing_shards" : 0,
  "unassigned_shards" : 0,
  "delayed_unassigned_shards" : 0,
  "number_of_pending_tasks" : 0,
  "number_of_in_flight_fetch" : 0,
  "task_max_waiting_in_queue_millis" : 0,
  "active_shards_percent_as_number" : 100.0
}
root@LAS:~# curl -XGET -u elastic:Z37ufZpU -k 'http://127.0.0.1:9200/_license' # ES 查询授权许可
{
  "license" : {
    "status" : "active",
    "uid" : "6db4d4ba-a409-43cc-9278-45e8043201ef",
    "type" : "basic",
    "issue_date" : "2022-05-18T12:33:36.765Z",
    "issue_date_in_millis" : 1652877216765,
    "max_nodes" : 1000,
    "issued_to" : "bigdata",
    "issuer" : "elasticsearch",
    "start_date_in_millis" : -1
  }
}
root@LAS:~# curl -XGET -u elastic:Z37ufZpU -k 'http://127.0.0.1:9200/_xpack/license' # ES 查询授权许可
{
  "license" : {
    "status" : "active",
    "uid" : "6db4d4ba-a409-43cc-9278-45e8043201ef",
    "type" : "basic",
    "issue_date" : "2022-05-18T12:33:36.765Z",
    "issue_date_in_millis" : 1652877216765,
    "max_nodes" : 1000,
    "issued_to" : "bigdata",
    "issuer" : "elasticsearch",
    "start_date_in_millis" : -1
  }
}
root@LAS:~# curl -u elastic:Z37ufZpU -k 'http://127.0.0.1:9200/_cat/indices' # ES查询索引
green open las-e-2022-08-01 P1hcLjCtRAmfumcUaTGnnA 1 0  57 0 134.1kb 134.1kb
green open las-e-2022-07-11 CHqhzA7ERzi5eBZveJlQVA 1 0  11 0 143.6kb 143.6kb
green open las-e-2022-07-22 sagFseupQBuSSvU-PUyIwQ 1 0  25 0 150.4kb 150.4kb
green open las-e-2022-07-21 S9uNdWdgTBSHWuvoybNqlg 1 0   4 0  40.4kb  40.4kb
green open las-e-2022-07-31 i1E9BltXQBaLT7VqWdFHbA 1 0   1 0  17.5kb  17.5kb
green open las-e-2022-07-09 vJfXM9ZdSsaPyDnHV_vQEA 1 0   2 0    19kb    19kb
green open las-e-2022-07-27 hoyAlYOLTw-yQJPhdyn0cg 1 0   3 0  39.6kb  39.6kb
green open las-e-2022-07-14 S9fXCk4uTDSglKFc3dYWzA 1 0  10 0  97.1kb  97.1kb
green open las-e-2022-07-13 njD_YAPASkmrGAQ6SsmLvQ 1 0   1 0  17.6kb  17.6kb
green open las-e-2022-08-04 IKXHOKgWSSSrDCcnny_S0A 1 0 182 0   170kb   170kb
green open las-e-2022-08-05 C5J7DQXnTFeerizk-Puz8g 1 0 416 0 201.3kb 201.3kb
green open las-e-2022-08-06 nkDGb_iASaSlf0VXWWaYHA 1 0 420 0 109.6kb 109.6kb
green open las-e-2022-08-07 JlMhcAOpTfSqeRrs1bT4kA 1 0 834 0 282.5kb 282.5kb
root@LAS:~# curl -XGET "http://127.0.0.1:9200/_cat/indices?v&pretty"
health status index            uuid                   pri rep docs.count docs.deleted store.size pri.store.size
green  open   las-e-2022-08-01 P1hcLjCtRAmfumcUaTGnnA   1   0         57            0    134.1kb        134.1kb
green  open   las-e-2022-07-22 sagFseupQBuSSvU-PUyIwQ   1   0         25            0    150.4kb        150.4kb
green  open   las-e-2022-07-11 CHqhzA7ERzi5eBZveJlQVA   1   0         11            0    143.6kb        143.6kb
green  open   las-e-2022-07-21 S9uNdWdgTBSHWuvoybNqlg   1   0          4            0     40.4kb         40.4kb
green  open   las-e-2022-07-31 i1E9BltXQBaLT7VqWdFHbA   1   0          1            0     17.5kb         17.5kb
green  open   las-e-2022-07-09 vJfXM9ZdSsaPyDnHV_vQEA   1   0          2            0       19kb           19kb
green  open   las-e-2022-07-27 hoyAlYOLTw-yQJPhdyn0cg   1   0          3            0     39.6kb         39.6kb
green  open   las-e-2022-07-14 S9fXCk4uTDSglKFc3dYWzA   1   0         10            0     97.1kb         97.1kb
green  open   las-e-2022-07-13 njD_YAPASkmrGAQ6SsmLvQ   1   0          1            0     17.6kb         17.6kb
green  open   las-e-2022-08-04 IKXHOKgWSSSrDCcnny_S0A   1   0        182            0      170kb          170kb
green  open   las-e-2022-08-05 C5J7DQXnTFeerizk-Puz8g   1   0        416            0    201.3kb        201.3kb
green  open   las-e-2022-08-06 nkDGb_iASaSlf0VXWWaYHA   1   0        420            0    109.6kb        109.6kb
green  open   las-e-2022-08-07 JlMhcAOpTfSqeRrs1bT4kA   1   0        855            0    262.9kb        262.9kb
root@LAS:~# curl -u elastic:Z37ufZpU -XDELETE 'http://127.0.0.1:9200/las-e-*/' -k # ES删除索引
{"acknowledged" : true}
root@LAS:~# curl -u elastic:Z37ufZpU 'http://127.0.0.1:9200/las-e-2022-08-07/_search?pretty' -k # ES查询指定索引内容
```

</details>

# es 备份、还原

```bash
1、创建备份目录并更改权限
执行以下命令创建备份文件存储的路径
mkdir -p /mount/backups/my_backup
更改权限以及属性
chmod 775 /mount/backups/my_backup
chown elasticsearch:elasticsearch /mount/backups/my_backup/
2、更改elasticsearch.yml文件
文件末尾增加
path.repo: ["/mount/backups/my_backup/"]
3、重启es并注册repository
重启命令:
/etc/init.d/elasticsearch restart
注册repository
curl -XPUT 'http://localhost:9200/_snapshot/backup' -d '{
"type":"fs",
"settings":{
"location":"/mount/backups/my_backup",
"compress":true
}
}'

4、查看是否注册成功
curl -XGET 'http://127.0.0.1:9200/_snapshot/backup'
返回结果：
{
"backup" : {
"type" : "fs",
"settings" : {
"compress" : "true",
"location" : "/usr/local/las/data/backup/es"
}
}
}

5、执行备份所有索引信息
curl -XPUT 'http://127.0.0.1:9200/_snapshot/backup/test1?wait_for_completion=true'
注释：此次备份名称为：test1,等待命令执行完毕，执行过程会消耗一段时间（需多等待一会）
6、备份指定索引信息
curl -XPUT 'http://127.0.0.1:9200/_snapshot/backup/esback' -d
'{
"indices":"las-e-2017-06-21,las-e-2016-07-21",
"ignore_unavailable":true,
"include_global_state":false,
"wait_for_completion":true
}'
7、查看备份状态
curl -XGET 'http://localhost:9200/_snapshot/backup/esback?pretty'
8、删除指定备份信息
curl -XDELETE 'http://localhost:9200/_snapshot/backup/esback'
9、还原所有备份信息
curl -XPOST 'http://localhost:9200/_snapshot/backup/bak/_restore'
10、还原指定索引信息
curl -XPOST 'http://localhost:9200/_snapshot/backup/bak/_restore' -d '{ "indices":"las-e2018-05-01,las-e-2018-05-02","ignore_unvailable":true}'
```

# 存在问题指令

执行备份所有索引信息-- > 存在问题
`curl -XPUT 'http://127.0.0.1:9200/_snapshot/backup/test1?wait_for_completion=true'`
ES查询索引中的字段类型-- > 存在问题
`curl -XGET "http://127.0.0.1:9200/productindex/_mapping?pretty"`
