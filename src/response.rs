//! A representation of a response from
//! the SocketLabs [Injection API](https://www.socketlabs.com/api-reference/injection-api/).

use std::borrow::Cow;

use serde::de::{Deserialize, Deserializer};

/// Representation of the SocketLabs AddressResult.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddressResult<'a> {
    /// The recipient address which generated the warning or error.
    pub email_address: Cow<'a, str>,
    /// A true or false value indicating whether or not
    /// the message was deliverable.
    pub accepted: bool,
    /// The reason for message delivery failure when an error
    /// occurs on the address-level.
    #[serde(deserialize_with = "deserialize_addressresult")]
    pub error_code: AddressResultErrorCode,
}

/// Representation of the SocketLabs MessageResult.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageResult<'a> {
    /// The index of the message that this response represents
    /// from the original array posted.
    pub index: u16,
    /// The reason for message delivery failure when an error
    /// occurs on the message-level.
    #[serde(deserialize_with = "deserialize_messageresult")]
    pub error_code: MessageResultErrorCode,
    /// An array of AddressResult objects that contain the status
    /// of each address that failed. If no messages failed this array is empty.
    pub address_result: Option<Vec<AddressResult<'a>>>,
}

/// Representation of the SocketLabs PostResponse.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<'a> {
    /// The success or failure details of the overall injection request.
    #[serde(deserialize_with = "deserialize_postmessage")]
    pub error_code: PostMessageErrorCode,
    /// A unique key generated if an unexpected error occurs during
    /// injection that can be used by SocketLabs support to
    /// troubleshoot the issue.
    pub transaction_receipt: Option<Cow<'a, str>>,
    /// An array of message result objects for messages that failed or
    /// have bad recipients. If there were no errors this response is empty.
    pub message_results: Option<Vec<MessageResult<'a>>>,
}

macro_rules! create_error_codes {
    ($(#[$docs:meta] ($enum:ident, $func: ident, ($(($kind:ident, $display:expr)),*) )),+) => ($(
        #[$docs]
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
    /// Return codes within the Response object, specifying the status of the injection request.
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
    /// Return codes within the MessageResult object, specifying the status of a specific message.
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
    /// Return codes within the AddressResult object, specifying the status of a specific recipient.
    (AddressResultErrorCode, deserialize_addressresult,
        ((InvalidAddress, "The address did not meet specification requirements."))
    )
}
