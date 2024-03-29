#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub public_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckUpdateRequest {
    pub tenant_name: String,
    pub version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckUpdateResponse {
    pub require_update: bool,
    pub url: String,
}
