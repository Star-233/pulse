# Pulse

轻量级校园网自动认证工具。告别开浏览器发心跳包的烦恼，用 Rust 写的轻量守护进程，自动完成网络认证与保活。

## 特性

- 极低资源占用，无浏览器依赖
- 自动检测本机 IP，完成 Portal 认证
- 定时心跳保活（每 5 秒发送 Session 探测）

## 下载

从 [Releases](https://github.com/anomalyco/pulse/releases) 下载对应平台的二进制文件，或自行构建：

| 平台 | 体积 |
|------|------|
| Linux (x86_64) | ~4.8 MB |
| Windows (x86_64) | ~7.0 MB |

## 构建

### Linux

```bash
cargo build --release
```

### Windows（交叉编译，需安装 MinGW）

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64        # Debian/Ubuntu
cargo build --release --target x86_64-pc-windows-gnu
```

## 使用

### 1. 创建配置文件 `config.toml`

```toml
[credentials]
user_id = "你的学号"
password = "你的密码"

[network]
wlanacip = "10.240.192.33"
wlanacname = "gxmzu"
mac = "00:00:00:00:00:00"
vlan = "0"
base_url = "https://aaa.gxmzu.edu.cn"
```

### 2. 运行

```bash
./pulse
```

首次运行没有配置文件会自动进入交互式设置，配置自动保存到 `~/.config/pulse/config.toml`。
