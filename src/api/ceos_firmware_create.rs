use core::panic;

use colored::Colorize;
use reqwest::header::{CONTENT_TYPE, COOKIE};

use crate::{
    structs::{
        args::CreateVersionParams,
        auth::AuthResponse,
        ceos_firmware::{ApiResponse, CreateVersionBody},
    },
    BASE_URL,
};

pub async fn create_ceos_firmware(
    client: &reqwest::Client,
    auth_data: &AuthResponse,
    params: CreateVersionParams,
) {
    let body = CreateVersionBody {
        tenant_name: params.tenant,
        version: params.version,
        checksum: params.md5,
        filename: params.path,
    };

    let res = client
        .post(BASE_URL.to_string() + "/ceos-firmware")
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
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
        reqwest::StatusCode::CREATED => {
            println!("Version {} created", body.version.purple())
        }
        reqwest::StatusCode::BAD_REQUEST => match res.json::<ApiResponse>().await {
            Ok(data) => println!("{}: {}", "Error".red(), data.message[0]),
            Err(_) => println!("{}: {}", "Error".red(), "Invalid input"),
        },
        _other => println!("{:#?}", res.text().await),
    }
}
