mod config;
mod auth;
mod heartbeat;

fn main() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Pulse - 校园网自动认证工具");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let config = config::Config::load().expect("❌ 加载配置失败");

    let wlanuserip = local_ip_address::local_ip()
        .expect("❌ 无法获取本机 IP 地址，请检查网络连接")
        .to_string();
    println!("🌐 本机 IP 地址：{}\n", wlanuserip);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .expect("❌ 创建网络客户端失败");

    auth::login(&client, &config, &wlanuserip).expect("❌ 认证失败，请检查账号密码");

    heartbeat::start(client, config, wlanuserip);
}
