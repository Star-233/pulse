use reqwest::blocking::Client;
use crate::config::Config;

const REDIRECT_URL: &str = "http://www.msftconnecttest.com/redirect";

pub fn login(client: &Client, config: &Config, wlanuserip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let login_url = format!(
        "{base}/webauth.do?wlanacip={acip}&wlanacname={acname}&wlanuserip={userip}&mac={mac}&vlan={vlan}&url={url}",
        base = config.network.base_url,
        acip = config.network.wlanacip,
        acname = config.network.wlanacname,
        userip = wlanuserip,
        mac = config.network.mac,
        vlan = config.network.vlan,
        url = REDIRECT_URL,
    );

    println!("⏳ 正在连接认证页面...");
    let _resp = client.get(&login_url).send()?;

    println!("⏳ 正在提交账号密码...");
    let params = [
        ("scheme", "https"),
        ("serverIp", "tomcat_server1:443"),
        ("hostIp", "http://127.0.0.1:8443/"),
        ("loginType", ""),
        ("auth_type", "0"),
        ("isBindMac1", "0"),
        ("pageid", "1"),
        ("templatetype", "1"),
        ("listbindmac", "0"),
        ("recordmac", "0"),
        ("isRemind", "1"),
        ("loginTimes", ""),
        ("groupId", ""),
        ("distoken", ""),
        ("echostr", ""),
        ("url", REDIRECT_URL),
        ("isautoauth", ""),
        ("mobile", ""),
        ("notice_pic_loop1", "/portal/uploads/pc/demo3/images/logo.jpg"),
        ("notice_pic_loop2", "/portal/uploads/pc/demo3/images/rrs_bg.jpg"),
        ("userId", &config.credentials.user_id),
        ("passwd", &config.credentials.password),
        ("remInfo", "on"),
        ("desc_lb", "on"),
    ];

    let resp = client.post(&login_url)
        .header("Referer", &login_url)
        .form(&params)
        .send()?;
    let body = resp.text()?;

    if body.contains("认证成功") || body.contains("LOGINSUCC") {
        println!("✅ 认证成功！已连接到校园网\n");
        Ok(())
    } else {
        Err("❌ 认证失败，返回的页面未显示成功信息".into())
    }
}
