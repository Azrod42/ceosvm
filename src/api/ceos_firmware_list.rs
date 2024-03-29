use core::panic;

use colored::Colorize;

use crate::{
    structs::{auth::AuthResponse, ceos_firmware::CeosFirmareListItem},
    BASE_URL,
};

pub async fn list_ceos_firmware(
    client: &reqwest::Client,
    auth_data: &AuthResponse,
    tenant_filter: &Option<String>,
) {
    let res = client
        .get(BASE_URL.to_string() + "/ceos-firmware")
        .header(
            "Cookie",
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
            match res.json::<Vec<CeosFirmareListItem>>().await {
                Ok(mut data) => {
                    if tenant_filter.is_some() {
                        data = data
                            .into_iter()
                            .filter(|item| item.tenant_name == tenant_filter.clone().unwrap())
                            .collect();
                    }
                    println!("{}", std::iter::repeat("-").take(97).collect::<String>());
                    println!(
                        "|  {}\t| {} | {}{}| {}{}| {} \t\t| {} \t\t|",
                        "id".purple(),
                        "stable".purple(),
                        " tenant".purple(),
                        std::iter::repeat(" ").take(9 - 7).collect::<String>(),
                        "     version".purple(),
                        std::iter::repeat(" ").take(20 - 12).collect::<String>(),
                        "checksum".purple(),
                        "filename".purple()
                    );
                    println!("{}", std::iter::repeat("-").take(97).collect::<String>());
                    for item in data {
                        println!("{}", item);
                    }
                    println!("{}", std::iter::repeat("-").take(97).collect::<String>());
                }
                Err(e) => panic!("{}", e.to_string()),
            };
        }
        other => println!("{}", other.to_string()),
    }
}
