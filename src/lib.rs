pub mod blocking;
pub mod dtypes;

pub const BASE_URL: &str = "https://api.openai.com/";

pub struct Client {
    pub base_url: String,
    pub api_key: String,
    pub org_id: Option<String>,
    pub req_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            api_key: api_key.to_string(),
            org_id: None,
            req_client: reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = Client::new("test");
    }
}
