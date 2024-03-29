use serde::{Deserialize, Serialize};

pub enum UserRole {
    ADMIN,
}

impl UserRole {
    pub fn check_role(&self, role: &String) -> bool {
        match self {
            UserRole::ADMIN => role == "admin",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthRoles {
    pub role_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthResponse {
    pub email: String,
    pub roles: Vec<AuthRoles>,
    pub tokens: AuthTokens,
    pub date: Option<String>,
}
