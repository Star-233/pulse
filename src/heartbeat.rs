use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use crate::config::Config;

const REDIRECT_URL: &str = "http://www.msftconnecttest.com/redirect";

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

    println!("Heartbeat started (interval: 5s, press Ctrl+C to stop)");

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
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Heartbeat OK");
                } else {
                    eprintln!("Heartbeat failed: HTTP {}", resp.status());
                }
            }
            Err(e) => {
                eprintln!("Heartbeat error: {}", e);
            }
        }
    }
}
