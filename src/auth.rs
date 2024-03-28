use reqwest::header::CONTENT_TYPE;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, Write};

#[derive(Serialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthRoles {
    roleName: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokens {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    email: String,
    roles: Vec<AuthRoles>,
    tokens: AuthTokens,
}

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

async fn auth_user(credential: Credential) -> Result<AuthResponse, ()> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:4000/v1/auth/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&credential)
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<AuthResponse>().await {
                Ok(parsed) => return Ok(parsed),
                Err(_) => return Err(()),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err(());
        }
        _other => return Err(()),
    }
}

pub async fn check_user_auth() -> Result<bool, String> {
    let _log = prompt_login();
    let log = Credential {
        email: "admin@smartandconnective.com".to_string(),
        password: "SnC2023!+1".to_string(),
    };

    let auth_response = auth_user(log).await.unwrap();

    if !auth_response
        .roles
        .iter()
        .any(|item| item.roleName == "admin")
    {
        return Err("Only admin can use this cli tool".to_string());
    }

    Ok(true)
}
