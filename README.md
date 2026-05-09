# Pulse

轻量级校园网自动认证工具。告别开浏览器发心跳包的烦恼，用 Rust 写的极小体积守护进程，自动完成网络认证与保活。

## 特性

- 极小资源占用，无浏览器依赖
- 自动检测网络连通性，断开时自动重连
- 支持定时心跳保活

## 构建

```bash
cargo build --release
```

## 使用

```bash
# 配置认证信息（编辑 config.toml）
# 运行
./target/release/pulse
```
