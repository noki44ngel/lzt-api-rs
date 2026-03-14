#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub page_count: i64,
    pub total: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Link {
    pub permalink: String,
    pub detail: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserSummary {
    pub user_id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<AvatarUrls>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AvatarUrls {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApiSystemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}
