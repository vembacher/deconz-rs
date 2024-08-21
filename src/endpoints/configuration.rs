use anyhow::{anyhow, Result};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    username: Option<String>,
    devicetype: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiToken {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub success: ApiToken,
}


impl TokenRequest {
    pub fn new(devicetype: String, username: Option<String>) -> Result<TokenRequest> {
        match &username {
            Some(username) => {
                if username.len() < 10 || username.len() > 40 {
                    return Err(anyhow!(
                        "invalid username length (is: {}, expected 10 <= len <= 40)",
                        username.len(),
                    ));
                }
            }
            None => {}
        }
        if devicetype.len() > 40 {
            Err(anyhow!(
                "invalid devicetype length (is: {}, expected len <= 40)",
                devicetype.len(),
            ))
        } else {
            Ok(TokenRequest {
                username,
                devicetype,
            })
        }
    }
}
