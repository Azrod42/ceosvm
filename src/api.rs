use crate::structs::auth::AuthResponse;

pub async fn list_ceos_firmware(auth_data: &AuthResponse) {
    let res = client
        .post("http://localhost:4000/v1/auth/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&credential)
        .send()
        .await;
}
