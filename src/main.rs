use std::env;
use std::process;

#[derive(serde::Deserialize)]
struct ApiResponse {
    success: bool,
    messages: Vec<String>,
}

fn parse_env(name: &str) -> String {
    env::var(name).unwrap_or_default()
}

fn validate_input(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
}

fn main() {
    let username = parse_env("DONDNS_USERNAME");
    let password = parse_env("DONDNS_PASSWORD");
    let hostname = parse_env("DONDNS_HOSTNAME");
    let address = parse_env("DONDNS_ADDRESS");

    if !validate_input(&username) {
        eprintln!("ERROR: DONDNS_USERNAME is invalid");
        process::exit(1);
    }
    if !validate_input(&password) {
        eprintln!("ERROR: DONDNS_PASSWORD is invalid");
        process::exit(1);
    }
    if !validate_input(&hostname) {
        eprintln!("ERROR: DONDNS_HOSTNAME is invalid");
        process::exit(1);
    }
    if !validate_input(&hostname) {
        eprintln!("ERROR: DONDNS_HOSTNAME is invalid");
        process::exit(1);
    }

    let mut params = vec![
        ("user", username.as_str()),
        ("password", password.as_str()),
        ("host", hostname.as_str()),
    ];
    if !address.is_empty() {
        params.push(("ip", address.as_str()));
    }

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://dondns.dondominio.com/json")
        .form(&params)
        .send()
        .unwrap_or_else(|e| {
            eprintln!("ERROR: request failed: {}", e.to_string().replace('\n', " "));
            process::exit(1);
        });

    let body: ApiResponse = response.json().unwrap_or_else(|e| {
        eprintln!("ERROR: failed to parse response: {}", e.to_string().replace('\n', " "));
        process::exit(1);
    });

    if body.success {
        println!("OK: {} updated", hostname);
    } else {
        eprintln!("ERROR: server failed: {}", body.messages.join(", ").replace('\n', " "));
        process::exit(1);
    }
}
