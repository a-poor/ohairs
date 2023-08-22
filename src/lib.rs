use anyhow::{anyhow, Result};
use reqwest::{Method, Request, RequestBuilder};
use serde::{Deserialize, Serialize};
use url::Url;

use dtypes::{ChatCompletionChunk, ChatCompletionObject, ChatCompletionRequest};

pub mod blocking;
pub mod dtypes;

#[cfg(feature = "mock")]
pub mod mock;

pub const BASE_URL: &str = "https://api.openai.com/";

pub struct Client {
    pub base_url: String,
    pub api_key: String,
    pub org_id: Option<String>,
    pub req_client: reqwest::Client,
}

impl Client {
    /// Create a new client with the given API key.
    ///
    /// The API key can be obtained from the [OpenAI dashboard](https://platform.openai.com/account/api-keys).
    ///
    /// # Example
    ///
    /// ```
    /// use ohairs::Client;
    ///
    /// let client = Client::new("test");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            api_key: api_key.to_string(),
            org_id: None,
            req_client: reqwest::Client::new(),
        }
    }

    fn format_url(&self, path: &str) -> Result<Url> {
        let base_url = Url::parse(self.base_url.as_str())
            .map_err(|err| anyhow!("Failed to parse self.base_url: {}", err))?;
        let url = base_url
            .join(path)
            .map_err(|err| anyhow!("Failed to add path to self.base_url: {}", err))?;
        Ok(url)
    }

    fn create_request(&self, method: reqwest::Method, path: &str) -> Result<RequestBuilder> {
        // Format the URL...
        let url = self.format_url(path)?;

        // Create a request builder...
        let mut req = self.req_client.request(method, url);

        // Add the auth header...
        req = req.bearer_auth(self.api_key.as_str());

        // If there's a org_id, add it...
        if let Some(org_id) = &self.org_id {
            req = req.header("OpenAI-Organization", org_id.as_str());
        }

        // Return the request...
        Ok(req)
    }

    pub async fn list_models(&self) -> Result<ListModelsResponse> {
        // Format the URL...
        let rb = self.create_request(Method::GET, "/v1/models")?;

        // Send the request...
        let res = rb
            .send()
            .await
            .map_err(|err| anyhow!("Failed to send request: {}", err))?;

        // TODO - Check status code and handle other possible states...

        // Parse the response as json...
        let data = res
            .json::<ListModelsResponse>()
            .await
            .map_err(|err| anyhow!("Failed to parse response as json: {}", err))?;

        // Return the data...
        Ok(data)
    }

    pub async fn create_chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject> {
        // Format the URL...
        let rb = self.create_request(Method::POST, "/v1/chat/completions")?;

        // Add the body...
        let rb = rb.json(&req);

        // Send the request...
        let res = rb
            .send()
            .await
            .map_err(|err| anyhow!("Failed to send request: {}", err))?;

        // TODO - Check status code and handle other possible states...

        // Parse the response as json...
        let data = res
            .json::<ChatCompletionObject>()
            .await
            .map_err(|err| anyhow!("Failed to parse response as json: {}", err))?;

        // Return the data...
        Ok(data)
    }

    // pub async fn create_chat_completion_stream(
    //     &self,
    //     req: ChatCompletionRequest,
    // ) -> Result<impl futures_core::Stream<Item = anyhow::Result<ChatCompletionChunk>>> {
    //     // Format the URL...
    //     let rb = self.create_request(Method::POST, "/v1/chat/completions")?;

    //     // Add the body...
    //     let rb = rb.json(&req);

    //     // Send the request...
    //     let res = rb
    //         .send()
    //         .await
    //         .map_err(|err| anyhow!("Failed to send request: {}", err))?;

    //     // TODO - Check status code and handle other possible states...

    //     // Parse the response as json...
    //     let data = res
    //         .json::<ChatCompletionObject>()
    //         .await
    //         .map_err(|err| anyhow!("Failed to parse response as json: {}", err))?;

    //     // Return the data...
    //     Ok(data)
    // }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<dtypes::ModelObject>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        let _ = Client::new("test");
        Ok(())
    }
}
