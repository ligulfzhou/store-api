# erp-api

## 如何运行
```
1: 安装postgresql, 创建erp数据库，并修改.env文件里的连续数据库用的账号信息
2: sqlx mig run（创建表结构）
3: 安装rust，并执行 cargo watch -q -c -w  src/ -x run
```
