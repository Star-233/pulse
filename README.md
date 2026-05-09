# Pulse

轻量级校园网自动认证工具。用 Rust 编写，告别开浏览器发心跳包的烦恼，自动完成校园网 Portal 认证与保活。

## 特性

- 极低资源占用，无浏览器依赖
- 自动检测本机 IP 完成认证
- 密码输入不显示在屏幕上
- 全中文交互界面，适合非技术用户
- 定时心跳保活（每 5 秒一次）
- 支持 Windows / Linux

## 下载

从 [Releases](https://github.com/Star-233/pulse/releases) 下载对应平台的程序，解压即可使用。

| 平台 | 体积 |
|------|------|
| Windows (x86_64) | ~7.0 MB |
| Linux (x86_64) | ~4.8 MB |

## 使用

### 方法一：交互式配置（推荐）

直接运行程序，按提示输入信息即可：

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Pulse - 校园网自动认证工具
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 未找到配置文件，开始设置...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  校园网认证配置向导
  如果某一步不清楚，直接回车使用默认值
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

请输入学号：
请输入密码：
AC IP 地址 [10.240.192.33]：
...
```

配置会自动保存到程序所在目录的 `config.toml`。

### 方法二：手动创建配置文件

在程序所在目录创建 `config.toml`：

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

然后双击运行程序即可。

## 自行构建

### Linux

```bash
cargo build --release
```

### Windows（交叉编译）

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64        # Debian/Ubuntu
cargo build --release --target x86_64-pc-windows-gnu
```
