use reqwest::blocking::Client;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::Config;

const REDIRECT_URL: &str = "http://www.msftconnecttest.com/redirect";

fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let hour = (secs / 3600 + 8) % 24; // UTC+8
    let min = (secs / 60) % 60;
    let sec = secs % 60;
    format!("{:02}:{:02}:{:02}", hour, min, sec)
}

pub fn start(client: Client, config: Config, wlanuserip: String) {
    let url = format!("{}/httpservice/updateSession.do", config.network.base_url);

    let referer = format!(
        "{base}/webauth.do?wlanacip={acip}&wlanacname={acname}&wlanuserip={userip}&mac={mac}&vlan={vlan}&url={redirect}",
        base = config.network.base_url,
        acip = config.network.wlanacip,
        acname = config.network.wlanacname,
        userip = wlanuserip,
        mac = config.network.mac,
        vlan = config.network.vlan,
        redirect = REDIRECT_URL,
    );

    println!("💓 心跳保活已启动");
    println!("⏰ 每 5 秒检测一次，网络正常时你会看到 ✅");
    println!("💡 请保持程序运行，关闭此窗口会导致网络断开");
    println!("💡 按 Ctrl+C 可安全退出\n");

    loop {
        thread::sleep(Duration::from_secs(5));

        match client.post(&url)
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", &referer)
            .form(&[
                ("pageid", "1"),
                ("userId", &config.credentials.user_id),
                ("wlanuserip", &wlanuserip),
            ]).send()
        {
            Ok(resp) if resp.status() == 200 => {
                println!("[{}] ✅ 心跳正常 · 网络连接正常", timestamp());
            }
            Ok(resp) => {
                eprintln!("[{}] ❌ 心跳异常：HTTP {}", timestamp(), resp.status());
            }
            Err(e) => {
                eprintln!("[{}] ❌ 心跳失败：{}", timestamp(), e);
            }
        }
    }
}
