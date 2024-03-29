use chrono::{DateTime, Duration, Local};
use colored::Colorize;
use reqwest::header::CONTENT_TYPE;
use rpassword::read_password;
use std::io::{self, BufRead, Write};

use crate::{
    file::{read_tmp_file, remove_tmp_file, store_token_in_tmp},
    structs::auth::{AuthResponse, Credential},
    utils::check_user_role,
    BASE_URL,
};

fn prompt_login() -> Credential {
    print!("Megahub email: ");
    std::io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().lock().read_line(&mut email).unwrap();
    email.pop();

    print!("Megahub password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap_or("".to_string());
    Credential { email, password }
}

async fn auth_user(credential: Credential) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();

    let res = client
        .post(BASE_URL.to_string() + "/auth/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&credential)
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(_) => panic!("{}: Unexpected error occurred", "Error".red()),
    };

    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<AuthResponse>().await {
                Ok(mut parsed) => {
                    parsed.date = Some(Local::now().to_string());
                    return Ok(parsed);
                }
                Err(e) => return Err(e.to_string()),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Unauthorized".to_string());
        }
        reqwest::StatusCode::BAD_REQUEST => {
            return Err("Invalid credential".to_string());
        }
        other => return Err(other.to_string()),
    }
}

pub fn verrify_cache(cache: Option<AuthResponse>) -> Option<AuthResponse> {
    match cache {
        Some(data) => {
            let date = data.clone().date.unwrap_or("".to_string());
            let token_date = DateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();
            let max_age = Local::now() - Duration::hours(1);
            if max_age > token_date {
                remove_tmp_file();
                return None;
            }
            Some(data)
        }
        None => None,
    }
}

pub async fn log_user() -> AuthResponse {
    let user = std::env::var("MEGAHUB_USER");
    let password = std::env::var("MEGAHUB_PASS");

    let mut log = Credential {
        email: String::from(""),
        password: String::from(""),
    };

    if user.is_ok() && password.is_ok() {
        log.email = user.unwrap_or(String::from(""));
        log.password = password.unwrap_or(String::from(""));
    } else {
        log = prompt_login();
    }

    let auth_response = auth_user(log)
        .await
        .unwrap_or_else(|e| panic!("Error: {}", e));
    check_user_role(&auth_response);
    store_token_in_tmp(&auth_response);
    auth_response
}

pub async fn get_auth_tokens(remove_cache: &bool) -> Option<AuthResponse> {
    let cache = read_tmp_file();
    match verrify_cache(cache) {
        Some(cache_data) => {
            if *remove_cache {
                remove_tmp_file();
                Some(log_user().await);
            }
            Some(cache_data)
        }
        None => Some(log_user().await),
    }
}
