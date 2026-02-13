# 离线模式

```shell
# 设置环境变量
export DATABASE_URL=postgres://tm_user:tm_password@localhost/tm_backend
# windows
# $env:DATABASE_URL = "postgres://tm_user:tm_password@localhost/tm_backend"

# 安装sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# 生成sqlx-data.json
cargo sqlx prepare
```