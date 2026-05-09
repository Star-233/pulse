mod config;
mod auth;
mod heartbeat;

fn main() {
    let config = config::Config::load().expect("Failed to load config");

    let wlanuserip = local_ip_address::local_ip()
        .expect("Failed to detect local IP")
        .to_string();
    println!("Local IP: {}", wlanuserip);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create HTTP client");

    auth::login(&client, &config, &wlanuserip).expect("Login failed");

    heartbeat::start(client, config, wlanuserip);
}
