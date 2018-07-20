use reqwest::{header::ContentType, Client, Response as ReqwestResponse};
use serde_json;

use error::{SocketLabsErrorKind, SocketLabsResult};
use message::Message;

static API_URL: &'static str = "https://inject.socketlabs.com/api/v1/email";

/// This is the struct that will hold
/// all SocketLabs tokens needed for
/// Injection API authentication and also
/// the vector with all the messages to send
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    server_id: u16,
    api_key: String,
    messages: Vec<Message>,
}

impl Request {
    /// Creates a new SocketLabs client with
    /// the given credentials
    pub fn new(
        server_id: u16,
        api_key: String,
        messages: Vec<Message>,
    ) -> SocketLabsResult<Request> {
        if messages.len() == 0 {
            return Err(SocketLabsErrorKind::MessageCountError.into());
        }

        Ok(Request {
            server_id: server_id,
            api_key: api_key,
            messages: messages,
        })
    }

    /// Sends an email using the SocketLabs Injection API
    pub fn send(&self) -> SocketLabsResult<ReqwestResponse> {
        let body = serde_json::to_string(&self)?;
        let client = Client::new();
        client
            .post(API_URL)
            .header(ContentType::json())
            .body(body)
            .send()
            .map_err(From::from)
    }
}
