use reqwest::Response as ReqwestResponse;
use serde::de::{Deserialize, Deserializer};
use serde_json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AddressResult {
    email_address: String,
    accepted: bool,
    #[serde(deserialize_with = "deserialize_addressresult")]
    error_code: AddressResultErrorCode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MessageResult {
    index: u16,
    #[serde(deserialize_with = "deserialize_messageresult")]
    error_code: MessageResultErrorCode,
    address_result: Option<Vec<AddressResult>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    #[serde(deserialize_with = "deserialize_postmessage")]
    pub error_code: PostMessageErrorCode,
    transaction_receipt: Option<String>,
    message_results: Option<Vec<MessageResult>>,
}

macro_rules! create_error_codes {
    ($(($enum:ident, $func: ident, ($(($kind:ident, $display:expr)),*) )),+) => ($(
        #[derive(Debug, Deserialize, Fail)]
        pub enum $enum {
            $(
                #[fail(display = $display)]
                $kind,
            )*
            #[fail(display = "SocketLabs returned an unknown error code.")]
            #[serde(skip_deserializing)]
            UnknownErrorCode,
        }

        fn $func<'de, D>(
            deserializer: D,
        ) -> Result<$enum, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok($enum::deserialize(deserializer)
                .unwrap_or($enum::UnknownErrorCode))
        }
    )*);
}

create_error_codes! {
    (PostMessageErrorCode, deserialize_postmessage,
        ((Success, "Success."),
        (Warning, "There were one or more failed messages and/or recipients."),
        (AccountDisabled, "The account has been disabled."),
        (InternalError, "Internal server error. (Please report to SocketLabs support if encountered.)"),
        (InvalidAuthentication, "The ServerId/ApiKey combination is invalid."),
        (InvalidData, "PostBody parameter does not have a valid structure, or contains invalid or missing data."),
        (NoMessages, "There were no messages to inject included in the request."),
        (EmptyMessage, "One or more messages have insufficient content to process."),
        (OverQuota, "Rate limit exceeded."),
        (TooManyErrors, "Authentication error limit exceeded."),
        (TooManyMessages, "Too many messages in a single request."),
        (TooManyRecipients, "Too many recipients in a single message."),
        (NoValidRecipients, "A merge was attempted, but there were no valid recipients."))
    ),
    (MessageResultErrorCode, deserialize_messageresult,
        ((Warning, "The message has one or more bad recipients."),
        (InvalidAttachment, "The message has one or more invalid attachments."),
        (MessageTooLarge, "The message was larger than the allowed size."),
        (EmptySubject, "This message contained an empty subject line, which is not allowed."),
        (EmptyToAddress, "This message does not contain a To address."),
        (InvalidFromAddress, "This message does not contain a valid From address."),
        (NoValidBodyParts, "This message does not have a valid text HTML body specified."),
        (NoValidRecipients, "There are no valid addresses specified as message recipients."),
        (InvalidMergeData, "The included merge data does not follow the API specification."),
        (InvalidTemplateId, "The selected API Template does not exist."),
        (MessageBodyConflict, "The Html Body and Text Body cannot be set when also specifying an API Template ID."))
    ),
    (AddressResultErrorCode, deserialize_addressresult,
        ((InvalidAddress, "The address did not meet specification requirements."))
    )
}

impl From<ReqwestResponse> for Response {
    fn from(mut response: ReqwestResponse) -> Response {
        serde_json::from_str::<Response>(&response.text().unwrap()).unwrap()
    }
}
