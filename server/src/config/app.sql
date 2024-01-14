-- 创建数据库
CREATE DATABASE unknown;

-- 连接到新创建的数据库
\c unknown;

-- 创建用户并设置密码
CREATE USER unknown WITH ENCRYPTED PASSWORD 'unknown-password';

-- 将用户赋予数据库的所有权限
GRANT ALL PRIVILEGES ON DATABASE unknown TO unknown;

-- 将用户赋予数据表的所有权限
GRANT ALL PRIVILEGES ON SCHEMA public TO unknown;