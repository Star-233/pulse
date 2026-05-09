use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use crate::config::Config;

pub fn start(client: Client, config: Config, wlanuserip: String) {
    let url = format!("{}/httpservice/updateSession.do", config.network.base_url);

    println!("Heartbeat started (interval: 5s, press Ctrl+C to stop)");

    loop {
        thread::sleep(Duration::from_secs(5));

        match client.post(&url).form(&[
            ("pageid", "1"),
            ("userId", &config.credentials.user_id),
            ("wlanuserip", &wlanuserip),
        ]).send() {
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
