use core::panic;

use colored::Colorize;
use reqwest::header::{CONTENT_TYPE, COOKIE};

use crate::{
    structs::{args::SetStableParams, auth::AuthResponse, ceos_firmware::ApiResponse},
    BASE_URL,
};

pub async fn update_ceos_firmware(
    client: &reqwest::Client,
    auth_data: &AuthResponse,
    params: SetStableParams,
) {
    let res = client
        .patch(format!(
            "{}/ceos-firmware?version={}&tenantName={}",
            BASE_URL, params.version, params.tenant
        ))
        .header(CONTENT_TYPE, "application/json")
        .header(
            COOKIE,
            format!(
                "Authentication={}; Refresh={}",
                &auth_data.tokens.access_token, &auth_data.tokens.refresh_token
            ),
        )
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(_) => panic!("Request fail"),
    };

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Version {} is now stable", params.version.purple())
        }
        reqwest::StatusCode::BAD_REQUEST => match res.json::<ApiResponse>().await {
            Ok(data) => println!("{}: {}", "Error".red(), data.message[0]),
            Err(_) => println!("{}: {}", "Error".red(), "Invalid input"),
        },
        _other => println!("{:#?}", res.text().await),
    }
}
