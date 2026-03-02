# Repository 测试说明

## 运行测试

### 设置测试环境

1. 确保你有一个 PostgreSQL 数据库用于测试
2. 设置环境变量或在 `.env` 文件中配置：

```bash
TEST_DATABASE_URL=postgresql://username:password@localhost:5432/it_service_test
```

### 运行所有测试

```bash
cd respository
cargo test
```

### 运行特定测试

```bash
# 运行用户 DAO 相关测试
cargo test users_dao

# 运行特定测试函数
cargo test test_create_user
```

## 测试内容

用户 DAO 测试包括以下功能：

- ✅ 创建用户 (`test_create_user`)
- ✅ 根据用户名查询用户 (`test_get_user_by_username`)
- ✅ 根据 ID 查询用户 (`test_get_user_by_id`)
- ✅ 更新用户信息 (`test_update_user`)
- ✅ 删除用户（软删除）(`test_delete_user`)
- ✅ 获取所有用户列表 (`test_get_all_users`)
- ✅ 边界情况测试 (`test_get_nonexistent_user`)

## 注意事项

1. 测试会使用 `TEST_DATABASE_URL` 环境变量指定的数据库
2. 如果未设置环境变量，将默认使用 `postgresql://postgres:postgres@localhost:5432/it_service_test`
3. 建议为测试创建独立的测试数据库，避免影响生产数据
4. 测试数据会在每次测试运行时创建和清理