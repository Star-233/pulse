use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub credentials: Credentials,
    pub network: Network,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub wlanacip: String,
    pub wlanacname: String,
    pub mac: String,
    pub vlan: String,
    pub base_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if std::fs::metadata(CONFIG_PATH).is_ok() {
            let content = std::fs::read_to_string(CONFIG_PATH)?;
            let config: Config = toml::from_str(&content)?;
            println!("✅ 已读取配置文件：{}", CONFIG_PATH);
            return Ok(config);
        }

        println!("📝 未找到配置文件，开始设置...\n");
        let config = Self::prompt()?;
        let toml_str = toml::to_string_pretty(&config)?;
        std::fs::write(CONFIG_PATH, &toml_str)?;
        println!("✅ 配置已保存到：{}（请勿分享此文件，内含密码）", CONFIG_PATH);
        Ok(config)
    }

    fn prompt() -> Result<Self, Box<dyn std::error::Error>> {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  校园网认证配置向导");
        println!("  如果某一步不清楚，直接回车使用默认值");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let user_id = prompt_input("请输入学号：");
        let password = prompt_password("请输入密码：");
        let wlanacip = prompt_input("AC  IP 地址（网关地址）[10.240.192.33]：");
        let wlanacip = if wlanacip.is_empty() { "10.240.192.33".into() } else { wlanacip };
        let wlanacname = prompt_input("AC 名称 [gxmzu]：");
        let wlanacname = if wlanacname.is_empty() { "gxmzu".into() } else { wlanacname };
        let mac = prompt_input("MAC 地址（可填 00:00:00:00:00:00）[00:00:00:00:00:00]：");
        let mac = if mac.is_empty() { "00:00:00:00:00:00".into() } else { mac };
        let base_url = prompt_input("认证服务器地址 [https://aaa.gxmzu.edu.cn]：");
        let base_url = if base_url.is_empty() { "https://aaa.gxmzu.edu.cn".into() } else { base_url };

        println!("\n✅ 信息已收集完毕，正在保存...\n");

        Ok(Config {
            credentials: Credentials { user_id, password },
            network: Network {
                wlanacip,
                wlanacname,
                mac,
                vlan: "0".into(),
                base_url,
            },
        })
    }
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_password(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    rpassword::read_password().unwrap_or_default()
}
