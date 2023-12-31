use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionRequest {
    /// ID of the model to use.
    ///
    /// See the [model endpoint compatibility](https://platform.openai.com/docs/models/model-endpoint-compatibility)
    /// table for details on which models work with
    /// the Chat API.
    pub model: String,

    /// A list of messages comprising the
    /// conversation so far.
    pub messages: Vec<ChatCompletionMessage>,

    /// A list of functions the model may generate
    // JSON inputs for.
    pub functions: Vec<Function>,

    /// Controls how the model responds to function calls.
    ///
    /// "none" means the model does not call a function, and
    /// responds to the end-user. "auto" means the model can
    /// pick between an end-user or calling a function.
    /// Specifying a particular function via `{"name":\ "my_function"}`
    /// forces the model to call that function. "none" is the
    /// default when no functions are present. "auto" is the
    /// default if functions are present.
    pub function_call: Option<FunctionCallType>,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused
    /// and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    ///
    /// Defaults to `1`.
    pub temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus
    /// sampling, where the model considers the results of the tokens
    /// with top_p probability mass. So 0.1 means only the tokens
    /// comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    ///
    /// Defaults to `1`.
    pub top_p: Option<f64>,

    /// How many chat completion choices to generate for each
    /// input message.
    ///
    /// Defaults to `1`.
    pub n: Option<u64>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    ///
    /// Tokens will be sent as data-only server-sent events as they
    /// become available, with the stream terminated by a `data: [DONE]`
    /// message.
    ///
    /// Defaults to `false`.
    pub stream: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    ///
    /// Defaults to `null`.
    pub stop: Option<StopToken>,

    /// The maximum number of tokens to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is
    /// limited by the model's context length.
    ///
    /// Defaulst to `inf`.
    pub max_tokens: Option<u64>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens
    /// based on whether they appear in the text so far, increasing the
    /// model's likelihood to talk about new topics.
    ///
    /// Defaults to `0`.
    pub presence_penalty: Option<f64>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens
    /// based on their existing frequency in the text so far, decreasing
    /// the model's likelihood to repeat the same line verbatim.
    ///
    /// Defaults to `0`.
    pub frequency_penalty: Option<f64>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token
    /// ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the
    /// model prior to sampling. The exact effect will vary per model, but
    /// values between -1 and 1 should decrease or increase likelihood of
    /// selection; values like -100 or 100 should result in a ban or
    /// exclusive selection of the relevant token.
    ///
    /// Defaults to `null`.
    pub logit_bias: Option<HashMap<String, f64>>,

    /// A unique identifier representing your end-user, which can help
    /// OpenAI to monitor and detect abuse.
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ModelObject {
    /// The model identifier, which can be referenced
    /// in the API endpoints.
    pub id: String,

    /// The object type, which is always "model".
    pub object: String,
    /// The date and time when the model was created.
    pub created: u64,

    /// The organization that owns the model.
    pub owned_by: String,
}

/// Represents a chat completion response returned
/// by model, based on the provided input.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionObject {
    /// A unique identifier for the chat completion.
    pub id: String,

    /// The object type, which is always `chat.completion`.
    pub object: String,

    /// A unix timestamp of when the chat completion
    /// was created.
    pub created: u64,

    /// The model used for the chat completion.
    pub model: String,

    /// A list of chat completion choices. Can be more
    /// than one if `n` is greater than `1`.
    pub choices: Vec<ChatCompletionChoice>,

    /// Usage statistics for the completion request.
    pub usage: ChatCompletionUsage,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionChoice {
    /// The index of the choice in the list of choices.
    pub index: u64,

    /// A chat completion message generated by the model.
    pub message: ChatCompletionMessage,

    /// The reason the model stopped generating tokens.
    ///
    /// This will be `stop` if the model hit a natural stop
    /// point or a provided stop sequence, `length` if the
    /// maximum number of tokens specified in the request
    /// was reached, or `function_call` if the model called
    /// a function.
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionMessage {
    /// The role of the author of this message.
    pub role: String,

    /// The contents of the message.
    ///
    /// content is required for all messages, and may
    /// be null for assistant messages with function calls.
    pub content: Option<String>,

    // /// The name of the author of this message.
    // ///
    // /// `name` is required if role is `function`, and
    // /// it should be the name of the function whose
    // /// response is in the `content`. May contain a-z,
    // /// A-Z, 0-9, and underscores, with a maximum length
    // /// of 64 characters.
    // pub name: Option<String>,
    /// The name and arguments of a function that should
    // be called, as generated by the model.
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct FunctionCall {
    /// The name of the function to call.
    pub name: String,

    /// The arguments to call the function with, as
    /// generated by the model in JSON format.
    ///
    /// Note that the model does not always generate valid
    /// JSON, and may hallucinate parameters not defined by
    /// your function schema. Validate the arguments in your
    /// code before calling your function.
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: u64,

    /// Number of tokens in the generated completion.
    pub completion_tokens: u64,

    /// Total number of tokens used in the request
    /// (prompt + completion).
    pub total_tokens: u64,
}

/// Represents a streamed chunk of a chat completion response
/// returned by model, based on the provided input.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ChatCompletionChunk {
    /// A unique identifier for the chat completion chunk.
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
}

impl ChatCompletionChunk {
    /// Tries to create a new `ChatCompletionChunk` from a
    /// string streamed from the API.
    ///
    /// The OpenAI API streams chat completion chunks with
    /// the prefix `data:`, followed by a JSON object. When
    /// the stream is finished, the API sends a (non-JSON)
    /// chunk `data: [DONE]`.
    ///
    /// This function will return `Ok(_)` if the chunk could
    /// be parsed, and `Err(_)` if the chunk could not be.
    /// It will return `Ok(None)` if the chunk was `data: [DONE]`
    /// and `Ok(Some(ChatCompletionChunk))` if the chunk was
    /// a valid JSON object.
    ///
    /// # Examples
    ///
    /// Checking that `data: {...}` is parsed correctly:
    ///
    /// ```
    /// use ohairs::dtypes::ChatCompletionChunk;
    /// let chunk = "data: {
    ///   \"id\": \"chatcmpl-123\",
    ///   \"object\": \"chat.completion.chunk\",
    ///   \"created\": 1677652288,
    ///   \"model\": \"gpt-3.5-turbo\",
    ///   \"choices\": []
    /// }";
    /// let data = ChatCompletionChunk::from_chunk(chunk);
    /// assert!(matches!(data, Ok(Some(ChatCompletionChunk(_)))));
    /// ```
    ///
    /// Checking that `data: [DONE]` is parsed correctly:
    ///
    /// ```
    /// use ohairs::dtypes::ChatCompletionChunk;
    /// let chunk = "data: [DONE]";
    /// let data = ChatCompletionChunk::from_chunk(chunk);
    /// assert!(matches!(data, Ok(None)));
    /// ```
    ///
    /// Checking that just JSON is parsed as an error:
    ///
    /// ```
    /// se ohairs::dtypes::ChatCompletionChunk;
    /// let chunk = "{
    ///   \"id\": \"chatcmpl-123\",
    ///   \"object\": \"chat.completion.chunk\",
    ///   \"created\": 1677652288,
    ///   \"model\": \"gpt-3.5-turbo\",
    ///   \"choices\": []
    /// }";
    /// let data = ChatCompletionChunk::from_chunk(chunk);
    /// assert!(data.is_err());
    /// ```
    pub fn from_chunk(chunk: &str) -> Result<Option<Self>> {
        // Strip any leading or trailing whitespace...
        let chunk = chunk.trim();

        // Does it start with `data:`?
        if !chunk.starts_with("data:") {
            return Err(anyhow!("Expected chunk to start with 'data:'"));
        }

        // Strip the `data:` prefix...
        let chunk = chunk
            .strip_prefix("data:")
            .ok_or(anyhow!("Expected chunk to have 'data:' prefix"))?
            .trim();

        // Is it `[DONE]`?
        if chunk == "[DONE]" {
            return Ok(None);
        }

        // Parse the chunk as JSON...
        let chunk: Self = serde_json::from_str(chunk)?;
        Ok(Some(chunk))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatCompletionChunkChoice {
    pub index: u64,
    pub delta: ChatCompletionMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Function {
    pub name: String,

    pub description: Option<String>,

    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum FunctionCallType {
    #[default]
    #[serde(rename = "none")]
    None,

    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "name")]
    Name(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StopToken {
    SingleToken(String),
    MultipleTokens(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chat_completion_chunk_from_chunk() {
        // Define cases to test...
        enum Res {
            IsErr,
            IsDone,
            IsChunk(ChatCompletionChunk),
        }
        let cases = vec![
            ("", Res::IsErr),
            ("{{", Res::IsErr),
            ("d a t a :", Res::IsErr),
            ("data: [DONE]", Res::IsDone),
            ("data: [DONE]\n\n", Res::IsDone),
            (
                "data: {
                    \"id\": \"\",
                    \"object\": \"\",
                    \"created\": 0,
                    \"model\": \"\",
                    \"choices\": []
                }",
                Res::IsChunk(ChatCompletionChunk {
                    ..Default::default()
                }),
            ),
            (
                "data: {
                    \"id\": \"chatcmpl-123\",
                    \"object\": \"chat.completion.chunk\",
                    \"created\": 1677652288,
                    \"model\": \"gpt-3.5-turbo\",
                    \"choices\": []
                }",
                Res::IsChunk(ChatCompletionChunk {
                    id: "chatcmpl-123".to_string(),
                    object: "chat.completion.chunk".to_string(),
                    created: 1677652288,
                    model: "gpt-3.5-turbo".to_string(),
                    ..Default::default()
                }),
            ),
            (
                "data: {
                    \"id\": \"chatcmpl-123\",
                    \"object\": \"chat.completion.chunk\",
                    \"created\": 1677652288,
                    \"model\": \"gpt-3.5-turbo\",
                    \"choices\": [
                        {
                            \"index\": 0,
                            \"finish_reason\": \"stop\",
                            \"delta\": {
                                \"role\": \"system\",
                                \"content\": \"You are a helpful assistant.\"
                            }
                        }
                    ]
                }",
                Res::IsChunk(ChatCompletionChunk {
                    id: "chatcmpl-123".to_string(),
                    object: "chat.completion.chunk".to_string(),
                    created: 1677652288,
                    model: "gpt-3.5-turbo".to_string(),
                    choices: vec![ChatCompletionChunkChoice {
                        index: 0,
                        finish_reason: Some("stop".to_string()),
                        delta: ChatCompletionMessage {
                            role: "system".to_string(),
                            content: Some("You are a helpful assistant.".to_string()),
                            ..Default::default()
                        },
                    }],
                }),
            ),
            (
                "data: {
                    \"id\": \"chatcmpl-123\",
                    \"object\": \"chat.completion.chunk\",
                    \"created\": 1677652288,
                    \"model\": \"gpt-3.5-turbo\",
                    \"choices\": [
                        {
                            \"index\": 0,
                            \"finish_reason\": \"stop\",
                            \"delta\": {
                                \"role\": \"assistant\",
                                \"content\": \"You are a helpful assistant.\"
                            }
                        },
                        {
                            \"index\": 1,
                            \"finish_reason\": \"length\",
                            \"delta\": {
                                \"role\": \"assistant\",
                                \"content\": \"You are a helpful assistant.\"
                            }
                        },
                        {
                            \"index\": 2,
                            \"finish_reason\": \"function_call\",
                            \"delta\": {
                                \"role\": \"assistant\",
                                \"function_call\": {
                                    \"role\": \"get_weather\",
                                    \"arguments\": \"{\\\"loc\\\": \\\"Los Angeles\\\"}\"
                                }
                            }
                        }
                    ]
                }",
                Res::IsChunk(ChatCompletionChunk {
                    id: "chatcmpl-123".to_string(),
                    object: "chat.completion.chunk".to_string(),
                    created: 1677652288,
                    model: "gpt-3.5-turbo".to_string(),
                    choices: vec![
                        ChatCompletionChunkChoice {
                            index: 0,
                            finish_reason: Some("stop".to_string()),
                            delta: ChatCompletionMessage {
                                role: "assistant".to_string(),
                                content: Some("You are a helpful assistant.".to_string()),
                                ..Default::default()
                            },
                        },
                        ChatCompletionChunkChoice {
                            index: 1,
                            finish_reason: Some("length".to_string()),
                            delta: ChatCompletionMessage {
                                role: "assistant".to_string(),
                                content: Some("You are a helpful assistant.".to_string()),
                                ..Default::default()
                            },
                        },
                        ChatCompletionChunkChoice {
                            index: 2,
                            finish_reason: Some("function_call".to_string()),
                            delta: ChatCompletionMessage {
                                role: "assistant".to_string(),
                                function_call: Some(FunctionCall {
                                    name: "get_weather".to_string(),
                                    arguments: "{\"loc\": \"Los Angeles\"}".to_string(),
                                }),
                                ..Default::default()
                            },
                        },
                    ],
                }),
            ),
        ];

        // Iterate over cases and test...
        for (i, (chunk, res)) in cases.into_iter().enumerate() {
            let data = ChatCompletionChunk::from_chunk(chunk);
            match res {
                Res::IsErr => assert!(data.is_err(), "expected case {} to be an error", i),
                Res::IsDone => {
                    assert!(
                        data.is_ok(),
                        "expected case {} to not be an error. Err: {}",
                        i,
                        data.unwrap_err()
                    );
                    assert!(matches!(data, Ok(None)), "expected case {} to be [DONE]", i)
                }
                Res::IsChunk(chunk) => {
                    assert!(
                        data.is_ok(),
                        "expected case {} to not be an error. Err: {}",
                        i,
                        data.unwrap_err(),
                    );
                    let data = data.unwrap();
                    assert!(data.is_some(), "expected case {} to not be none", i);
                    assert_eq!(data, Some(chunk), "expected case {} to be a chunk", i);
                }
            }
        }
    }
}
