# 🚀elasticsearch 文件说明

## 1. elasticsearch 配置文件说明

```yaml
elasticsearch: 
 bin:
 lib:
 modules:
 logs:
 plugins:
 config:
  elasticsearch.yml              #  elasticsearch 配置文件
  jvm.options                      # jvm 配置文件
  log4j2.properties             # 日志配置文件
```

## 2. jvm.options参数

```bash
################################################################
## IMPORTANT: JVM heap size
################################################################
##
## The heap size is automatically configured by Elasticsearch
## based on the available memory in your system and the roles
## each node is configured to fulfill. If specifying heap is
## required, it should be done through a file in jvm.options.d,
## and the min and max should be set to the same value. For
## example, to set the heap to 4 GB, create a new file in the
## jvm.options.d directory containing these lines:
##
## -Xms4g # 默认4g
## -Xmx4g
##
## See https://www.elastic.co/guide/en/elasticsearch/reference/current/heap-size.html
## for more information
##
################################################################
```

## 3.elasticsearch.yml

```yaml
# 集群使用描述性名称:
cluster.name: my-application
# 使用节点的描述性名称:
node.name: node-1
# 存放数据的目录的路径(多个位置用逗号分隔)
path.data: /path/to/data
# 日志文件路径
path.logs: /path/to/logs
# 默认情况下Elasticsearch只能在本地主机上访问。在这里设置一个不同的地址来公开网络上的这个节点:
network.host: 192.168.0.1
# 默认 http 端口 9200
http.port: 9200

# 配置跨域
http.cors.enabled: true
http.cors.allow-origin: "*"
```

## 4. 访问`curl "http://127.0.0.1:9200"`

```bash
[root@8f1930bdze131 ~]# curl "http://127.0.0.1:9200"
{
  "name" : "8f1930bdze131", # 主机名
  "cluster_name" : "elasticsearch", # 默认一个也是集群，默认集群名 elasticsearch
  "cluster_uuid" : "XjL5BIXbRrOY0VR4HfloEQ", # 集群 uuid
  "version" : {
    "number" : "7.12.0", # 当前版本
    "build_flavor" : "default",
    "build_type" : "docker",
    "build_hash" : "78722783c38caa25a70982b5b042074cde5d3b3a",
    "build_date" : "2021-03-18T06:17:15.410153305Z",
    "build_snapshot" : false,
    "lucene_version" : "8.8.0", # 基于 lucene 版本
    "minimum_wire_compatibility_version" : "6.8.0",
    "minimum_index_compatibility_version" : "6.0.0-beta1"
  },
  "tagline" : "You Know, for Search" # 标语：你知道了，为了收索
}

```

# docker 安装🔨 elasticsearch

## 1. 拉取镜像

```bash
sudo docker pull elasticsearch:7.12.0
```

## 2. 创建docker容器挂载目录

```bash
sudo mkdir -pv /opt/elasticsearch/config
sudo mkdir -pv /opt/elasticsearch/data
sudo mkdir -pv /opt/elasticsearch/plugins
```

`mkdir` 参数说明：

- -p, --parents     如果存在，则没有错误，根据需要创建父目录
- -v, --verbose     为每个创建的目录打印一条消息

## 3. 配置文件（elasticsearch.yml）

```bash
echo "http.host: 0.0.0.0" > /opt/elasticsearch/config/elasticsearch.yml
chmod -R 777 /opt/elasticsearch/
```

参数说明：

- ">"                          如果文件存在，清空文件内容并写入。如果文件不存在，创建新文件并写入。

- -R, --recursive        递归地更改文件和目录

- 777                         可读、可写、可执行权限

## 4. 创建容器

```bash
sudo docker run --name elasticsearch -p 9200:9200  -p 9300:9300 \
 -e "discovery.type=single-node" \
 -e ES_JAVA_OPTS="-Xms84m -Xmx512m" \
 -v /opt/elasticsearch/config/elasticsearch.yml:/usr/share/elasticsearch/config/elasticsearch.yml \
 -v /opt/elasticsearch/data:/usr/share/elasticsearch/data \
 -v /opt/elasticsearch/plugins:/usr/share/elasticsearch/plugins \
 -d elasticsearch:7.12.0
```

`docker run`参数说明:

- --name string                    为容器分配一个名称
- -p, --publish list                  向主机发布容器的端口
- -P, --publish-all                    将所有公开的端口发布到随机端口
- -e discovery.type=single-node 单点模式启动
- -e ES_JAVA_OPTS="-Xms84m -Xmx512m"：设置启动占用的内存范围
- -v, --volume list                    绑定挂载卷
- -d, --detach                         在后台运行容器并打印容器ID

## 5. 查看启动详情

```bash
docker ps  查看是否启动
docker logs elasticsearch  启动日志查询
docker restart elasticsearch   重启
docker exec -it elasticsearch bash 进入
```

**操作明细**

```bash
[root@localhost ~]# docker pull elasticsearch:7.12.0
7.12.0: Pulling from library/elasticsearch
7a0437f04f83: Pull complete 
2b674c951ca3: Pull complete 
06baeb69f25f: Pull complete 
eeff01d19ce5: Pull complete 
a994306398ca: Pull complete 
2c002d76c1f6: Pull complete 
6286f2196f9b: Pull complete 
Digest: sha256:383e9fb572f3ca2fdef5ba2edb0dae2c467736af96aba2c193722aa0c08ca7ec
Status: Downloaded newer image for elasticsearch:7.12.0
docker.io/library/elasticsearch:7.12.0
[root@localhost ~]# docker images
REPOSITORY      TAG       IMAGE ID       CREATED         SIZE
elasticsearch   7.12.0    9337ed510a0c   18 months ago   830MB
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/config
mkdir: 已创建目录 "/opt/elasticsearch"
mkdir: 已创建目录 "/opt/elasticsearch/config"
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/data
mkdir: 已创建目录 "/opt/elasticsearch/data"
[root@localhost opt]# sudo mkdir -pv /opt/elasticsearch/plugins
mkdir: 已创建目录 "/opt/elasticsearch/plugins"
[root@localhost config]# echo "http.host: 0.0.0.0" > /opt/elasticsearch/config/elasticsearch.yml
[root@localhost config]# sudo docker run --name elasticsearch -p 9200:9200  -p 9300:9300 \
>  -e "discovery.type=single-node" \
>  -e ES_JAVA_OPTS="-Xms84m -Xmx512m" \
>  -v /opt/elasticsearch/config/elasticsearch.yml:/usr/share/elasticsearch/config/elasticsearch.yml \
>  -v /opt/elasticsearch/data:/usr/share/elasticsearch/data \
>  -v /opt/elasticsearch/plugins:/usr/share/elasticsearch/plugins \
>  -d elasticsearch:7.12.0
8f1930bde13101b5f0412d2e31c7ebc9114c80d95b36da4ead466262574642af
[root@localhost ~]# docker ps
CONTAINER ID   IMAGE                  COMMAND                  CREATED          STATUS             PORTS                                                                                  NAMES
8f1930bde131   elasticsearch:7.12.0   "/bin/tini -- /usr/l…"   13 minutes ago   Up 5 minutes       0.0.0.0:9200->9200/tcp, :::9200->9200/tcp, 0.0.0.0:9300->9300/tcp, :::9300->9300/tcp   elasticsearch

[root@localhost ~]# curl "http://127.0.0.1:9200"
{
  "name" : "8f1930bde131",
  "cluster_name" : "elasticsearch",
  "cluster_uuid" : "XjL5BIXbRrOY0VR4HfloEQ",
  "version" : {
    "number" : "7.12.0",
    "build_flavor" : "default",
    "build_type" : "docker",
    "build_hash" : "78722783c38caa25a70982b5b042074cde5d3b3a",
    "build_date" : "2021-03-18T06:17:15.410153305Z",
    "build_snapshot" : false,
    "lucene_version" : "8.8.0",
    "minimum_wire_compatibility_version" : "6.8.0",
    "minimum_index_compatibility_version" : "6.0.0-beta1"
  },
  "tagline" : "You Know, for Search"
}
```

# 安装 elasticsearch-ik 分词器

==elasticsearch-ik 分词器版本和 elasticsearch 版本必须一致==

## 1. 拉取安装包

```sh
wget https://github.com/medcl/elasticsearch-analysis-ik/releases/download/v7.12.0/elasticsearch-analysis-ik-7.12.0.zip
```

## 2. 创建 ik 目录

```sh
mkdir -pv /opt/elasticsearch/plugins/ik/
unzip elasticsearch-analysis-ik-7.12.0.zip # 解压到ik目录中
```

## 3.重启服务

```sh
docker restart elasticsearch
```

# 🔨kibana 安装

## 可视化界面

 [Elasticvue - Microsoft Edge Addons](https://microsoftedge.microsoft.com/addons/detail/elasticvue/geifniocjfnfilcbeloeidajlfmhdlgo)

## 1. 安装kibana

==kibana，elasticsearch需要版本一致==

```sh
wget https://artifacts.elastic.co/downloads/kibana/kibana-7.12.0-linux-x86_64.tar.gz
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-7.12.0-linux-x86_64.tar.gz
```

## 2. 配置`kibana/config/kibana.yml`

```sh
server.host: "192.168.1.102"
# 配置远程服务器地址
elasticsearch.hosts: ["http://192.168.1.102:9200"]
# 设置中文
i18n.locale: "zh-CN"
# zh-CN
/usr/local/kibana/x-pack/plugins/translations/translations/zh-CN.json
```

## 3. 启动 kibana

```sh
# kibana 不支持root用户启动
./kibana/bin/kibana --allow-root &
```

## 4. 测试连接

`systemctl stop firewalld.service # 关闭防火墙`

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008090548309-87994082.png)

`web访问：http://192.168.1.102:5601`

# 📝elasticsearch 自定义字典

## 1. 配置词典

```sh
[root@localhost config]# pwd
/opt/elasticsearch/plugins/ik/config
[root@localhost config]# cat username.dic # 配置自己字典
柳宗林
[root@localhost config]# cat IKAnalyzer.cfg.xml 
﻿<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE properties SYSTEM "http://java.sun.com/dtd/properties.dtd">
<properties>
 <comment>IK Analyzer 扩展配置</comment>
 <!--用户可以在这里配置自己的扩展字典 -->
 <entry key="ext_dict">username.dic</entry>
  <!--用户可以在这里配置自己的扩展停止词字典-->
 <entry key="ext_stopwords"></entry>
 <!--用户可以在这里配置远程扩展字典 -->
 <!-- <entry key="remote_ext_dict">words_location</entry> -->
 <!--用户可以在这里配置远程扩展停止词字典-->
 <!-- <entry key="remote_ext_stopwords">words_location</entry> -->
</properties>
[root@localhost config]# 
```

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008125621315-1053577461.png)

## 2. 重启 elasticsearch

docker 容器重启会报错，重启 docker 服务

**索引操作（Restful风格）**

![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008130041614-1723779801.png)
![image](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221008130152834-1439983600.png)

ik_smart 最少切分

```json
# 创建分词器
# get 请求，_analyze 分词

GET _analyze
{
  "analyzer": "ik_smart",
  "text": "Elasticsearch是Elastic Stack核心的分布式搜索和分析引擎。Logstash和Beats有助于收集，聚合和丰富您的数据并将其存储在Elasticsearch中。使用Kibana，您可以交互式地探索，可视化和共享对数据的见解，并管理和监视堆栈。Elasticsearch是建立索引，搜索和分析魔术的地方"
}
```

ik_max_word 最细粒度划分

```json
GET _analyze
{
  "analyzer": "ik_max_word",
  "text": "Elasticsearch是Elastic Stack核心的分布式搜索和分析引擎。Logstash和Beats有助于收集，聚合和丰富您的数据并将其存储在Elasticsearch中。使用Kibana，您可以交互式地探索，可视化和共享对数据的见解，并管理和监视堆栈。Elasticsearch是建立索引，搜索和分析魔术的地方"
}
```

## 把“柳宗林” 添加到 ik 分词器字典当中

```json
GET _analyze
{
  "analyzer": "ik_max_word",
  "text": "柳宗林"
}

# 柳宗琳 未添加到分词器当中
GET _analyze
{
  "analyzer": "ik_max_word",
  "text": "柳宗琳"
}
```

# 📦RestFul 风格

## ✨put一个索引

```json
# Rest 风格
# 创建一个索引
# put /索引名/类型名称/文档id {请求体}
PUT /test1/type1/1
{
  "name": "liuzonglin",
  "age": 18
}
```

## put _doc 默认类型

```json
PUT /test3/_doc/1
{
  "name": "liuzonglin",
  "age": 18,
  "birth": "1997-07-27"
}
```

## put 修改索引

```json
# put 修改索引 覆盖原有信息 更新版本（?v）每次 put 都会跟 _version 信息
PUT /test3/_doc/1
{
  "name": "liuzonglin",
  "age": 18,
  "birth": "1997-07-27"
}
```

## put创建索引规则

```json
# 创建一个库
# 方法体 创建索引规则
PUT /test2
{
  "mappings": {
    "properties": {
      "name": {
        "type": "text"
      },
      "age": {
        "type": "long"
      },
      "birthday": {
        "type": "date"
      }
    }
  }
}
```

## GET 查询索引

```json
# 查询索引
# http://192.168.1.102:9200/test1/type1/1
# curl "http://127.0.0.1:9200/test1/type1/1"
GET /test1/type1/1
GET test3/_doc/1
```

## GET 获取索引的信息

```json
GET test2
```

## GET 获取 elasticsearch 健康值

```json
GET _cat/health
```

## GET 查看所有索引库版本信息

```json
GET _cat/indices?v
```

## post 修改文档

```json
POST /test3/_doc/1/_update
{
  "doc": {
    "name": "liuzonglin",
    "age": 25
  }
}
```

## DELETE 删除索引

```json
DELETE /test1
```

# 📝查询练习

## 1. 写入数据

```json
# 文档操作
PUT /liuzonglin/user/1
{
  "name": "liuzonglin",
  "age": 26,
  "desc": "天天都在学习，还算有点进步",
  "tages": ["技术仔", "好色", "指南"]
  
}

PUT /liuzonglin/user/2
{
  "name": "张三",
  "age": 26,
  "desc": "还算有点进步",
  "tages": ["技术仔", "渣男"]
  
}

PUT /liuzonglin/user/3
{
  "name": "李四",
  "age": 26,
  "desc": "还算有点进步",
  "tages": ["技术仔"]
  
}


```

## 2. 查询数据

```json
GET _cat/indices?v
GET /liuzonglin/user/3
```

## 3. 更新数据

```json
# 文档更新操作不加”_update“会跟put 命令一样效果，覆盖原有数据
POST /liuzonglin/user/1/
{
  "doc": {
    "name": "liuzonglin",
    "age": 26,
    "desc": "天天都在学习，还算有点进步",
    "tages": ["技术仔", "擅长java", "指南学习"]
  }
}

POST /liuzonglin/user/1/_update
{
  "doc": {
    "name": "liuzonglin",
    "age": 26,
    "desc": "天天都在学习，还算有点进步",
    "tages": ["技术仔", "擅长java", "指南学习"]
  }
}

PUT /liuzonglin/user/1
{
  "name": "张三",
  "age": 26,
  "desc": "还算有点进步",
  "tages": ["技术仔", "渣男"]
}
```

## get 查询某个字段

```json
GET liuzonglin/_search
{
  "query": {
    "match": {
      "name": "棕"
    }
  }
}

# 错误语句：不支持在搜索请求中指定类型。
GET liuzonglin/_doc/_search
{
  "query": {
    "match": {
      "name": "棕"
    }
  }
}
```

## get 通过 age 进行排序

```json
GET liuzonglin/_search
{
  "query": {
    "match": {
      "name": "棕"
    }
  },
  "sort": [
    {
      "age": {
        "order": "desc" # 降序 ; asc 升序 
      }
    }
  ]
}
```

## GET 分页查询

```json
GET liuzonglin/_search
{
  "query": {
    "match": {
      "name": "棕"
    }
  },
  "sort": [
    {
      "age": {
        "order": "desc"
      }
    }
  ],
  "from": 0, # 从第几个开始
  "size": 20 # 每页面数据个数
}
```

数据索引下标还是从 0 开始的

## GET 布尔值查询（多条件精确查询）

**1. must (and) 所有条件都要符合**

```json
GET liuzonglin/_search
{
  "query": {
    "bool": {
      "must": [
        {
          "match": {
            "name": "刘" # 模糊匹配
          }
        },
        {
          "match": {
            "age": 26 # age 相当于唯一标识
          }
        }
      ]
    }
  }
}
```

**2. should（or）只要满足一条**

```json
GET liuzonglin/_search
{
  "query": {
    "bool": {
      "should": [
        {
          "match": {
            "name": "刘" # 模糊匹配
          }
        },
        {
          "match": {
            "age": 26 # age 相当于唯一标识
          }
        }
      ]
    }
  }
}
```

**3. must_not（no）返回不满足的信息**

```json
GET liuzonglin/_search
{
  "query": {
    "bool": {
      "must_not": [
        {
          "match": {
            "age": 2 # age 相当于唯一标识
          }
        }
      ]
    }
  }
}
```

**4. filter 数据过滤**

```json
GET liuzonglin/_search
{
  "query": {
    "bool": {
      "must": [
        {
          "match": {
            "name": "刘"
          }
        },
        {
          "match": {
            "age": 26
          }
        }
      ],
      "filter": [
        {
          "range": {
            "age": {
              "gte": 10, # 大于等于 10
              "lte": 20  # 小于等于 20
            }
          }
        }
      ]
    }
  }
}
```

参数说明：

- gt    大于
- gte   大于等于
- lt      小于
- lte    小于等于

**5. 匹配多个条件**

```json
GET liuzonglin/_search
{
  "query": {
    "match": {
      "tags": "天 三"
    }
  }
}
```

多个参数用空格分开

**6. term 精确查询**

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009124320282-944259033.png)

**关于分词：**

- term 直接精确查询
- match 使用分词器解析（先分析文档，然后通过分析的文档进行查询）

**text 可以分分词 keyword 不会被分词**

**7. term 精确多个值查询**

```json
GET liuzonglin/_search
{
  "query": {
    "bool": {
      "should": [
        {
          "term": {
            "t1": 2 
          }
        },
        {
          "term": {
            "t1": 3 
          }
        }
      ]
    }
  }
}
```

**8. 高亮查询**

```json
GET liuzonglin/_search
{
  "query": {
    "match": {
      "name": "棕"
    }
  },
  "highlight": {
    "fields": {
      "name": {}
    }
  }
}
```

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009130634993-805532018.png)

# ES curl 常用指令

## 查询所有节点

```sh
root@LAS:~# curl 'http://127.0.0.1:9200/_cat/nodes' # 查询所有节点
192.168.31.127 49 61 0 2.16 2.11 2.03 dilmrt * node-1
```

## 查询集群状态

```sh
root@LAS:~# curl -k 'http://127.0.0.1:9200/_cluster/health?pretty' # 查询集群状态
{
  "cluster_name" : "elasticsearch",
  "status" : "yellow",
  "timed_out" : false,
  "number_of_nodes" : 1,
  "number_of_data_nodes" : 1,
  "active_primary_shards" : 14,
  "active_shards" : 14,
  "relocating_shards" : 0,
  "initializing_shards" : 0,
  "unassigned_shards" : 7,
  "delayed_unassigned_shards" : 0,
  "number_of_pending_tasks" : 0,
  "number_of_in_flight_fetch" : 0,
  "task_max_waiting_in_queue_millis" : 0,
  "active_shards_percent_as_number" : 66.66666666666666
}
```

## GET 查询授权许可

```sh
root@LAS:~# curl -k 'http://127.0.0.1:9200/_license' # ES 查询授权许可
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
```

## GET 查询授权许可

```sh
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
```

## GET 查询索引（ip 为真实ip）

```sh
root@LAS:~# curl -k 'http://127.0.0.1:9200/_cat/indices' # ES查询索引
green open las-e-2022-08-01 P1hcLjCtRAmfumcUaTGnnA 1 0  57 0 134.1kb 134.1kb
green open las-e-2022-07-11 CHqhzA7ERzi5eBZveJlQVA 1 0  11 0 143.6kb 143.6kb
green open las-e-2022-07-22 sagFseupQBuSSvU-PUyIwQ 1 0  25 0 150.4kb 150.4kb

root@LAS:~# curl -XGET "http://127.0.0.1:9200/_cat/indices?v&pretty"
health status index            uuid                   pri rep docs.count docs.deleted store.size pri.store.size
green  open   las-e-2022-08-01 P1hcLjCtRAmfumcUaTGnnA   1   0         57            0    134.1kb        134.1kb
green  open   las-e-2022-07-22 sagFseupQBuSSvU-PUyIwQ   1   0         25            0    150.4kb        150.4kb
green  open   las-e-2022-07-11 CHqhzA7ERzi5eBZveJlQVA   1   0         11            0    143.6kb        143.6kb
```

## GET 查询指定 id

```sh
# liuzonglin_jd1 文档索引（_index）
# _doc 文档类型（_type）
# 10 文档 id (_id)
curl -XGET  'http://127.0.0.1:9200/liuzonglin_jd1/_doc/10/?pretty' -k
```

## POST 查询分页数据

```sh
# from 开启，size 显示条数
[root@localhost ~]# curl -XPOST http://127.0.0.1:9200/liuzonglin_jd1/_search?pretty -H 'content-Type:application/json' -d '{"query":{"match_all":{}},"from":0,"size":10}'
```

## ES curl 查询指定索引内容

```bash
root@LAS:~# curl 'http://127.0.0.1:9200/las-e-2022-08-07/_search?pretty' -k # ES查询指定索引内容
```

## PUT 创建索引

```sh
# liuzonglin 索引（index）
curl -XPUT http://127.0.0.1:9200/liuzonglin?pretty
```

## PUT 创建文档

```sh
curl -XPUT http://127.0.0.1:9200/liuzonglin/_doc/2?pretty -H 'content-Type:application/json' -d '{"name":"liuzonglin","age":"26"}'
```

## ES curl 删除全部索引

```sh
# las-e-* 全部索引名（index）
root@localhost:~# curl -XDELETE 'http://127.0.0.1:9200/las-e-*/' -k # ES删除索引
{"acknowledged" : true}
```

## DELETE 删除指定索引 id

```sh
[root@localhost ~]# curl -XDELETE 'http://127.0.0.1:9200/liuzonglin_jd1/_doc/10/?pretty' -k
{
  "_index" : "liuzonglin_jd1",
  "_type" : "_doc",
  "_id" : "10",
  "found" : false
}
```

# ES 备份、还原

## 1. 创建备份目录并更改权限

```sh
执行以下命令创建备份文件存储的路径
mkdir -p /mount/backups/my_backup
更改权限以及属性
chmod 775 /mount/backups/my_backup
chown elasticsearch:elasticsearch /mount/backups/my_backup/
```

## 2. 更改 elasticsearch.yml 文件

```sh
文件末尾增加
path.repo: ["/mount/backups/my_backup/"]
```

## 3. 重启 ES 并注册repository

```sh
重启命令:
/etc/init.d/elasticsearch restart
注册repository
curl -XPUT 'http://localhost:9200/_snapshot/backup' -d '{
 "type": "fs",
 "settings": {
  "location": "/mount/backups/my_backup",
  "compress": true
 }
}'
```

## 4. 查看是否注册成功

```sh
curl -XGET 'http://127.0.0.1:9200/_snapshot/backup'
返回结果：
{
 "backup": {
  "type": "fs",
  "settings": {
   "compress": "true",
   "location": "/usr/local/las/data/backup/es"
  }
 }
}
```

## 5. 执行备份所有索引信息

```sh
curl -XPUT 'http://127.0.0.1:9200/_snapshot/backup/test1?wait_for_completion=true'
注释：此次备份名称为：test1,等待命令执行完毕，执行过程会消耗一段时间（需多等待一会）
```

## 6. 备份指定索引信息

```sh
curl -XPUT 'http://127.0.0.1:9200/_snapshot/backup/esback' -d
'{
 "indices": "las-e-2017-06-21,las-e-2016-07-21",
 "ignore_unavailable": true,
 "include_global_state": false,
 "wait_for_completion": true
}'
```

## 7. 查看备份状态

```sh
curl -XGET 'http://localhost:9200/_snapshot/backup/esback?pretty'
```

## 8. 删除指定备份信息

```sh
curl -XDELETE 'http://localhost:9200/_snapshot/backup/esback'
```

## 9. 还原所有备份信息

```sh
curl -XPOST 'http://localhost:9200/_snapshot/backup/bak/_restore'
```

## 10. 还指定索引信息

```bash
curl -XPOST 'http://localhost:9200/_snapshot/backup/bak/_restore' -d '{ "indices":"las-e2018-05-01,las-e-2018-05-02","ignore_unvailable":true}'
```

# elasticsearch 集群搭建

## elasticsearch.yml

```yml
cluster.name: bigdata
node.name: node-1
path.data: /usr/local/las/data/elasticsearch
path.logs: /usr/local/las/log/elasticsearch
bootstrap.memory_lock: false
bootstrap.system_call_filter: false
network.host: 0.0.0.0
network.publish_host: 172.100.19.155
http.port: 9200
http.max_content_length: 100mb
http.cors.enabled: true
discovery.seed_hosts: ["172.100.19.155","172.100.19.156"]
cluster.initial_master_nodes: ["172.100.19.155","172.100.19.156"]
node.master: true
node.data: true
transport.tcp.port: 9300
transport.tcp.compress: true
indices.fielddata.cache.size: 10%
path.repo: ["/usr/local/las/data/backup/es"]
indices.query.bool.max_clause_count: 10240
```

## 参考文档

<https://blog.csdn.net/qq_50227688/article/details/115379121>

# spring boot 整合 ES

官网地址api文档地址：<https://www.elastic.co/guide/en/elasticsearch/client/index.html>

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009143553314-1978524853.png)

**1. 依赖引入**

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009144428713-712918718.png)

**2. 初始化对象**

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009144545010-411497571.png)

**3. 创建项目导入依赖**

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009150057029-23054127.png)

**导入依赖版本和实际使用版本不一致（ES 版本必须一致）**

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009150454833-574821360.png)

**自定义版本**
![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009152412599-2006939674.png)

**源码**
![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009155049526-1244278333.png)
![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221009155708230-399251058.png)

## ES api 测试

### 1. 初始化 ES 创建索引

```java
package com.example.es.config;

import org.apache.http.HttpHost;
import org.elasticsearch.client.RestClient;
import org.elasticsearch.client.RestHighLevelClient;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/*
  ES配置

  @author liuzonglin
 * @date 2022/10/09
 */

/**
 * 配置配置类注解
 * @author liuzonglin
 */
@Configuration
public class ElasticsearchConfig {

    /**
     * 配置 Bean
     * spring <beans id="restHighLevelClient" class="RestHighLevelClient"/>
     */
    @Bean
    public RestHighLevelClient restHighLevelClient() {
        return new RestHighLevelClient(
                RestClient.builder(
                        // 集群配置多个 new HttpHost("localhost", 9201, "http")
                        new HttpHost("192.168.1.102", 9200, "http")));
    }
}

```

### 2. 创建 查询 删除 （索引 文档）

```java
package com.example.es.pojo;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.stereotype.Component;

/**
 * 用户
 *
 * @author liuzonglin
 * @date 2022/10/09
 */

/**
 * Component 注入 spring 中
 * AllArgsConstructor 有参
 * NoArgsConstructor 无参
 */
@Component
@Data
@AllArgsConstructor
@NoArgsConstructor
public class User {
    private String name;
    private Integer age;
}

```

```java
package com.example.es.utis;

/**
 * elasticsearch 工具栏
 *
 * @author liuzonglin
 * @date 2022/10/09
 */
public class ElasticsearchUtils {
    public final static String ES_INDEX = "liuzonglin_index";

}

```

```java
package com.example.es;

import com.alibaba.fastjson.JSON;
import com.example.es.pojo.User;
import com.example.es.utis.ElasticsearchUtils;
import lombok.SneakyThrows;
import org.elasticsearch.action.admin.indices.delete.DeleteIndexRequest;
import org.elasticsearch.action.bulk.BulkRequest;
import org.elasticsearch.action.bulk.BulkResponse;
import org.elasticsearch.action.delete.DeleteRequest;
import org.elasticsearch.action.delete.DeleteResponse;
import org.elasticsearch.action.get.GetRequest;
import org.elasticsearch.action.get.GetResponse;
import org.elasticsearch.action.index.IndexRequest;
import org.elasticsearch.action.index.IndexResponse;
import org.elasticsearch.action.search.SearchRequest;
import org.elasticsearch.action.search.SearchResponse;
import org.elasticsearch.action.support.master.AcknowledgedResponse;
import org.elasticsearch.action.update.UpdateRequest;
import org.elasticsearch.action.update.UpdateResponse;
import org.elasticsearch.client.RequestOptions;
import org.elasticsearch.client.RestHighLevelClient;
import org.elasticsearch.client.indices.CreateIndexRequest;
import org.elasticsearch.client.indices.CreateIndexResponse;
import org.elasticsearch.client.indices.GetIndexRequest;
import org.elasticsearch.common.unit.TimeValue;
import org.elasticsearch.common.xcontent.XContentType;
import org.elasticsearch.index.query.QueryBuilders;
import org.elasticsearch.index.query.TermQueryBuilder;
import org.elasticsearch.search.SearchHit;
import org.elasticsearch.search.builder.SearchSourceBuilder;
import org.elasticsearch.search.fetch.subphase.FetchSourceContext;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.boot.test.context.SpringBootTest;

import java.io.IOException;
import java.util.ArrayList;
import java.util.concurrent.TimeUnit;


@SpringBootTest
class EsApplicationTests {

    /**
     * 客户端
     * Autowired 默认选择他的一个类型
     * client 不是 RestHighLevelClient 的默认类型 名字问题，如果需要这样设置需要加 @Qualifier("restHighLevelClient")
     */
    @Autowired
    @Qualifier("restHighLevelClient")
    private RestHighLevelClient client;

    /**
     * 创建索引
     */
    @SneakyThrows
    @Test
    void createIndex() {
        // 创建索引请求
        CreateIndexRequest request = new CreateIndexRequest("liuzonglin_index");
        // 执行索引请求 indicesClient 请求后获得响应
        CreateIndexResponse response = client.indices().create(request, RequestOptions.DEFAULT);
        System.out.println("response = " + response);

    }

    /**
     * 查询索引
     */
    @SneakyThrows
    @Test
    void queryIndex() {
        // 获取索引请求
        GetIndexRequest request = new GetIndexRequest("liuzonglin_index");
        boolean exists = client.indices().exists(request, RequestOptions.DEFAULT);
        System.out.println(exists);
    }


    /**
     * 删除索引
     */
    @Test
    void deleteIndex() throws IOException {
        // 获取删除索引请求
        DeleteIndexRequest request = new DeleteIndexRequest("liuzonglin_index2");
        AcknowledgedResponse delete = client.indices().delete(request, RequestOptions.DEFAULT);
        System.out.println(delete.isAcknowledged());

    }

    /**
     * 创建文档
     */
    @SneakyThrows
    @Test
    void createDocument() {
        // 创建对象
        User user = new User("liuzonglin", 26);
        // 创建请求
        IndexRequest request = new IndexRequest("liuzonglin_index");
        // 设置规则
        request.id("1");
        // 设置过期时间 1s
        request.timeout(TimeValue.timeValueSeconds(1));
        request.timeout("1s");

        // 放入请求数据 json
        // 引入 str 转 josn 的依赖
        request.source(JSON.toJSONString(user), XContentType.JSON);
        // 客户端发送请求, 获取相应结果
        IndexResponse index = client.index(request, RequestOptions.DEFAULT);
        System.out.println(index.toString());
        System.out.println(index.status());
    }

    /**
     * 得到文档
     * get/liuzonglin/_doc/1
     */
    @SneakyThrows
    @Test
    void getDocument() {
        // 获取请求
        GetRequest getRequest = new GetRequest("liuzonglin_index", "1");
        // 不获取返回的 _source 的上下文
        getRequest.fetchSourceContext(new FetchSourceContext(false));
        getRequest.storedFields("_none_");
        boolean exists = client.exists(getRequest, RequestOptions.DEFAULT);
        System.out.println(exists);
    }

    /**
     * 获取文件信息
     */
    @SneakyThrows
    @Test
    void getDocumentInformation() {
        GetRequest getRequest = new GetRequest("liuzonglin_index", "1");
        GetResponse getResponse = client.get(getRequest, RequestOptions.DEFAULT);
        // 打印文档
        System.out.println(getResponse.getSourceAsString());
        System.out.println(getResponse.getSource());
        System.out.println(getRequest.version());
    }

    /**
     * 更新文档
     */
    @SneakyThrows
    @Test
    void updateDocument() {
        UpdateRequest updateRequest = new UpdateRequest("liuzonglin_index", "1");
        updateRequest.timeout("1s");
        User user = new User("liuzonglin1", 27);
        updateRequest.doc(JSON.toJSONString(user), XContentType.JSON);
        UpdateResponse update = client.update(updateRequest, RequestOptions.DEFAULT);
        System.out.println(update);
    }

    /**
     * 删除文档
     */
    @SneakyThrows
    @Test
    void deleteDocument() {
        DeleteRequest deleteRequest = new DeleteRequest("liuzonglin_index", "1");
        deleteRequest.timeout("1s");
        DeleteResponse delete = client.delete(deleteRequest, RequestOptions.DEFAULT);
        System.out.println(delete);
    }

    /**
     * 批量插入
     */// 批量插入数据
    @SneakyThrows
    @Test
    void bulkInsert() {
        BulkRequest bulkRequest = new BulkRequest();
        bulkRequest.timeout("100s");
        ArrayList<User> list = new ArrayList<User>() {
            {
                this.add(new User("liuzonglin1", 1));
                this.add(new User("liuzonglin2", 2));
                this.add(new User("liuzonglin3", 3));
                this.add(new User("liuzonglin4", 4));
                this.add(new User("liuzonglin5", 5));
                this.add(new User("liuzonglin6", 6));
                this.add(new User("liuzonglin7", 7));
                this.add(new User("liuzonglin8", 8));
                this.add(new User("liuzonglin9", 9));
                this.add(new User("liuzonglin10", 10));
            }
        };
        int size = list.size();
        for (int i = 0; i < size; i++) {
            bulkRequest.add(
                    new IndexRequest("liuzonglin_index")
                    .id(" " + (i + 1))
                    .source(JSON.toJSONString(list.get(i)),XContentType.JSON));
        }
        BulkResponse bulk = client.bulk(bulkRequest, RequestOptions.DEFAULT);
        // 是否失败
        System.out.println(bulk.hasFailures());
    }

    /**
     * 获取文件信息
     */
    @SneakyThrows
    @Test
    void batchQuery() {
        // 1. 搜索请求
        SearchRequest searchRequest = new SearchRequest(ElasticsearchUtils.ES_INDEX);
        // 2. 搜索条件构建
        SearchSourceBuilder searchSourceBuilder = new SearchSourceBuilder();
        // QueryBuilders.termQuery(); 快速查询精确匹配
        // QueryBuilders.matchAllQuery(); 匹配所有
        TermQueryBuilder termQueryBuilder = QueryBuilders.termQuery("name", "liuzonglin");
        searchSourceBuilder.query(termQueryBuilder);
        searchSourceBuilder.timeout(new TimeValue(60, TimeUnit.SECONDS));
        // 放入请求
        searchRequest.source(searchSourceBuilder);
        // 执行请求
        SearchResponse search = client.search(searchRequest, RequestOptions.DEFAULT);
        System.out.println(JSON.toJSONString(search.getHits()));
        for (SearchHit documentFields : search.getHits().getHits()) {
            System.out.println(documentFields);
        }
    }
}


```

## 实战案例

> 导入依赖

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221010054916981-1326200322.png)

> 导入 fastjson 数据

```xml
<dependency>
    <groupId>com.alibaba</groupId>
    <artifactId>fastjson</artifactId>
    <version>1.2.83</version>
</dependency>
```

注意导入的 ES 版本

> 导入静态文件

> 测试

```java
package com.example.elasticsearch.controller;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

/**
 * 指数控制器
 *
 * @author liuzonglin
 * @date 2022/10/10
 */
@Controller
public class IndexController {

    /**
     * 指数
     * getMapping 需要返回一个页面 index ({})
     * @return {@link String}
     */
    @GetMapping({"/", "index"})
    public String index() {
        return "index";
    }
}

```

> 导入 jsoup（获取请求返回的页面信息，帅选出想要的数据）

```xml
<!-- tika 爬取视频音乐-->
<!-- 解析网页 jsoup 爬取网页-->
<dependency>
    <groupId>org.jsoup</groupId>
    <artifactId>jsoup</artifactId>
    <version>1.11.3</version>
</dependency>
```

> 爬取数据

```java
package com.example.elasticsearch.utils;

import com.example.elasticsearch.pojo.Content;
import lombok.SneakyThrows;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;
import org.jsoup.select.Elements;

import java.net.URL;
import java.util.ArrayList;
import java.util.List;

/**
 * 解析网页
 *
 * @author liuzonglin
 * @date 2022/10/10
 */
public class HtmlParseUtil {

    public static void main(String[] args) {
        new HtmlParseUtil().parsingPage("java").forEach(System.out::println);
    }


    /**
     * 解析页面
     */
    @SneakyThrows
    public List<Content> parsingPage(String keywords) {
        // 联网 获取请求
        String url = "https://search.jd.com/Search?keyword=" + keywords;
        // 解析网页 Jsoup 返回 document(整个文档页面) 浏览器对象
        Document document = Jsoup.parse(new URL(url), 3000);
        // 根据 id 查找 页面中所需信息
        Element element = document.getElementById("J_goodsList");
        // System.out.println(element.html());
        // 获取所有元素
        Elements li = element.getElementsByTag("li");

        ArrayList<Content> contents = new ArrayList<>();


        // 或取元素中内容
        for (Element l : li) {
            // 获取图片 无法获取图片 source-data-lazy-img
            String img = l.getElementsByTag("img").eq(0).attr("data-lazy-img");
            // 商品价格
            String price = l.getElementsByClass("p-price").eq(0).text();
            // 商品名称
            String title = l.getElementsByClass("p-name").eq(0).text();

            Content content = new Content();
            content.setImg(img);
            content.setTitle(title);
            content.setPrice(price);
            contents.add(content);
        }
        return contents;
    }

}

```

> 封装数据

```java
package com.example.elasticsearch.pojo;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.stereotype.Component;

/**
 * 内容
 *
 * @author liuzonglin
 * @date 2022/10/10
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
@Component
public class Content {
    private String title;
    private String img;
    private String price;
}

```

![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221010070052644-1518107587.png)
![img](https://img2022.cnblogs.com/blog/2402369/202210/2402369-20221010064637863-1222952285.png)

```java
package com.example.elasticsearch.config;

import org.apache.http.HttpHost;
import org.elasticsearch.client.RestClient;
import org.elasticsearch.client.RestHighLevelClient;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/*
  ES配置

  @author liuzonglin
 * @date 2022/10/09
 */

/**
 * 配置配置类注解
 * @author liuzonglin
 */
@Configuration
public class ElasticsearchConfig {

    /**
     * 配置 Bean
     * spring <beans id="restHighLevelClient" class="RestHighLevelClient"/>
     */
    @Bean
    public RestHighLevelClient restHighLevelClient() {
        return new RestHighLevelClient(
                RestClient.builder(
                        new HttpHost("192.168.1.103", 9200, "http")));
    }
}

```

```java
package com.example.elasticsearch.controller;

import com.example.elasticsearch.service.ContentService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Map;

/**
 * 内容控制器
 *
 * @author liuzonglin
 * @date 2022/10/10
 */
@RestController
public class ContentController {

    @Autowired
    private ContentService contentService;


    /**
     * ES 批量创建文档
     *
     * @param keywords 关键字
     * @return {@link Boolean}
     */
    @GetMapping("/parse/{keywords}")
    public Boolean parse(@PathVariable("keywords") String keywords) {
        return contentService.parseContent(keywords);
    }


    /**
     * 搜索
     *
     * @param pageNo   页面没有
     * @param pageSize 页面大小
     * @param keyword  关键字
     * @return {@link List}<{@link Map}<{@link String}, {@link Object}>>
     */
    @GetMapping("/search/{keyword}/{pageNo}/{pageSize}")
    public List<Map<String, Object>> search(@PathVariable String keyword,
                                            @PathVariable int pageNo,
                                            @PathVariable int pageSize) {
        return contentService.searchPage(keyword, pageNo, pageSize);
    }
}

```

```java
package com.example.elasticsearch.service;

import com.example.elasticsearch.pojo.Content;
import com.example.elasticsearch.utils.HtmlParseUtil;
import lombok.SneakyThrows;
import org.elasticsearch.action.bulk.BulkRequest;
import org.elasticsearch.action.bulk.BulkResponse;
import org.elasticsearch.action.index.IndexRequest;
import org.elasticsearch.action.search.SearchRequest;
import org.elasticsearch.action.search.SearchResponse;
import org.elasticsearch.client.RequestOptions;
import org.elasticsearch.client.RestHighLevelClient;
import org.elasticsearch.common.unit.TimeValue;
import org.elasticsearch.common.xcontent.XContentType;
import org.elasticsearch.index.query.QueryBuilders;
import org.elasticsearch.index.query.TermQueryBuilder;
import org.elasticsearch.search.SearchHit;
import org.elasticsearch.search.builder.SearchSourceBuilder;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;

/**
 * 内容服务
 *
 * @author liuzonglin
 * @date 2022/10/10
 */
@Service
public class ContentService {

    /**
     * 客户端
     */
    @Autowired
    private RestHighLevelClient client;



    /**
     * 解析内容
     * 解析数据放入 ES 当中
     *
     * @param keywords 关键字
     * @return {@link Boolean}
     */
    @SneakyThrows
    public Boolean parseContent(String keywords) {
        // 爬取数据
        List<Content> contents = new HtmlParseUtil().parsingPage(keywords);
        // 数据放入 ES 当中
        BulkRequest bulkRequest = new BulkRequest();
        bulkRequest.timeout("2m");
        for (Content content : contents) {
            bulkRequest.add(new IndexRequest("liuzonglin_jd")
                    .source(content, XContentType.JSON));
        }
        // 批量创建文档
        BulkResponse bulk = client.bulk(bulkRequest, RequestOptions.DEFAULT);
        // 是否成功
        return !bulk.hasFailures();

    }

    /**
     * 搜索页面
     * 搜索数据分页
     *
     * @param keyword  关键字
     * @param pageNo   页面没有
     * @param pageSize 页面大小
     * @return {@link List}<{@link Map}<{@link String}, {@link Object}>>
     */
    @SneakyThrows
    public List<Map<String, Object>> searchPage(String keyword, int pageNo, int pageSize) {
        // 如果页面数量小于等于 1
        if (pageNo <= 1) {
            pageNo = 1;
        }
        // 条件查询
        SearchRequest searchRequest = new SearchRequest("liuzonglin_jd");
        SearchSourceBuilder sourceBuilder = new SearchSourceBuilder();
        // 分页
        sourceBuilder.from(pageNo);
        sourceBuilder.size(pageSize);
        // 精准匹配
        TermQueryBuilder termQueryBuilder = QueryBuilders.termQuery("title", keyword);
        sourceBuilder.query(termQueryBuilder);
        sourceBuilder.timeout(new TimeValue(60, TimeUnit.SECONDS));
        // 执行搜索
        searchRequest.source(sourceBuilder);
        SearchResponse search = client.search(searchRequest, RequestOptions.DEFAULT);

        // 解析结果
        ArrayList<Map<String, Object>> list = new ArrayList<>();
        for (SearchHit hit : search.getHits().getHits()) {
            System.out.println("hit = " + hit);
            list.add(hit.getSourceAsMap());

        }
        return list;
    }
}

```
