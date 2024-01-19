```sh
#!/bin/bash

#
# MongoDB 数据导入导出统一 json 格式工具
# 适用于 CentOS7 环境
# 使用说明：
# 请参考以下步骤操作：
# 1. 导入数据： `/bin/bash mongodb_.sh export_collection`
# 2. 导出数据： `/bin/bash mongodb_.sh import_collection`
# 3. 支持的文件格式：json
# 4. 其他说明：无
#

# 定义集合名 14灌机包
SETNAME=()

# 创建存储目录
mkdir -p /root/admin-mongodb

# 导出集合
export_collection() {

    for ((i = 0; i < ${#SETNAME[*]}; i++)); do
        /usr/local/las/program/mongodb/bin/mongoexport -d admin -c "${SETNAME[i]}" -o /root/admin-mongodb/"${SETNAME[i]}".json -u root -p mongodb@cl0vdsec.c0m
    done
}

# 导入集合
import_collection() {

    for ((i = 0; i < ${#SETNAME[*]}; i++)); do
        /usr/local/las/program/mongodb/bin/mongoimport -d admin -c "${SETNAME[i]}" --file /root/admin-mongodb/"${SETNAME[i]}".json -u root -p mongodb@cl0vdsec.c0m
    done
}

case "$1" in
export_collection)
    export_collection
    ;;
import_collection)
    import_collection
    ;;
*)

    echo "bash mongodb_.sh { export_collection | import_collection }" >&2
    exit 1
    ;;

esac
exit 0
```
