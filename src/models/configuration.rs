use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    token: String,
    channel_name: String
}

impl Configuration {
    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_channel_name(&self) -> &str {
        &self.channel_name
    }
}

impl std::default::Default for Configuration {
    fn default() -> Self {
        Self { token: "".to_string(), channel_name: "".to_string() }
    }
}

