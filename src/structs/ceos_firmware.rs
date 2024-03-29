use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CeosFirmareListItem {
    pub id: i32,
    pub version: String,
    pub checksum: String,
    pub is_stable: bool,
    pub tenant_name: String,
    pub filename: String,
}

impl Display for CeosFirmareListItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut status = '❌';
        if self.is_stable {
            status = '✅'
        }

        write!(
            f,
            "| {}\t|   {}   | {}{}| {}{}| {} | {} |",
            self.id,
            status,
            self.tenant_name,
            std::iter::repeat(" ")
                .take(9 - self.tenant_name.len())
                .collect::<String>(),
            self.version,
            std::iter::repeat(" ")
                .take(20 - self.version.len())
                .collect::<String>(),
            self.checksum,
            self.filename
        )
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVersionBody {
    pub version: String,
    pub checksum: String,
    pub tenant_name: String,
    pub filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub status_code: u16,
    pub message: Vec<String>,
    pub error: String,
}
