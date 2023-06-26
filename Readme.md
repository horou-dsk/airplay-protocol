## 注意事项

引用本项目在debug模式启动，需要在cargo.toml配置

```toml
[profile.dev]
overflow-checks = false # 去除数值溢出检查
```