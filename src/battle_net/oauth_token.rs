#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl OAuthToken {
    pub fn new(access_token: String, token_type: String, expires_in: u64) -> OAuthToken {
        OAuthToken { access_token, token_type, expires_in, }
    }
}
