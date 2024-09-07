# Ecredit

`Ecredit`是一个灵活、可扩展的通用积分管理系统，支持多种积分类型和使用场景，帮助用户轻松管理虚拟积分。

## 概述

Ecredit提供了一种灵活且通用的解决方案，用于管理和追踪虚拟积分。无论在何种业务场景下，都能通过支持多种积分类型，帮助企业和用户更加轻松地整合和管理虚拟资产。

## 使用指南

1. 克隆仓库。
2. 根据需要自定义`.env`和`config.toml`文件中的配置。
3. 执行数据库迁移。

   ```bash
   # 确保已安装`sqlx-cli`。可以使用以下命令进行安装：
   cargo install sqlx-cli

   # 确保`.env`文件中配置了正确的数据库连接信息。
   DATABASE_URL=postgres://username:password@host/database

   # 在DATABASE_URL处创建数据库：
   sqlx database create

   # 执行数据迁移命令以设置数据库表：
   sqlx migrate run
   ```

4. 使用`cargo run`启动应用程序。
5. 通过指定的地址访问应用程序，并使用提供的API功能。

## 最低支持Rust版本

Ecredit支持的最低Rust版本为1.70。

## 许可协议

本项目使用[MIT许可证](LICENSE)授权。
