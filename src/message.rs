use emailaddress::EmailAddress;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// This is a representation of email attachments
/// that corresponds to the way SocketLabs represents them
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Attachment {
    /// The name of the attachment
    name: String,
    /// A description of the content in the attachment
    content: String,
    /// The id of the content in the attachment
    content_id: String,
    /// The type of the content in the attachment
    content_type: String,
    /// The headers in the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<Vec<CustomHeaders>>,
}

/// This is a representation of email headers
/// that corresponds to the way SocketLabs represents them
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CustomHeaders {
    /// The name of the header
    name: String,
    /// The value of the header
    value: String,
}

/// This is a representation of an email address
/// plus the optional name of the owner of that address
#[derive(Debug)]
pub struct Email {
    /// The actual email address
    email_address: EmailAddress,
    /// The name of the owner of the address
    friendly_name: Option<String>,
}

// Had to implement this manually,
// because EmailAddress doesn't implement Serialize by default.
impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Email", 2)?;
        state.serialize_field("EmailAddress", &self.email_address.to_string())?;
        if let Some(ref friendly_name) = self.friendly_name {
            state.serialize_field("FriendlyName", friendly_name)?;
        }
        state.end()
    }
}

impl Email {
    pub fn new(email_address: String, friendly_name: Option<String>) -> Email {
        let email_address = EmailAddress::new(&email_address).unwrap();
        Email {
            email_address: email_address,
            friendly_name: friendly_name,
        }
    }
}

/// This is a representation of the data storage for the
/// inline Merge feature from SocketLabs. More about it:
/// https://www.socketlabs.com/blog/unleash-power-merge-fields/
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MergeData {
    /// A vector used to define merge field data for each
    /// message. Variables can be freely named, with the
    /// exception of a single reserved word, `DeliveryAddress`
    /// which defines the recipient of the current message
    per_message: Vec<Data>,
    /// A vector used to define merge field data for all
    /// messages in the injection
    global: Vec<Data>,
}

/// Helper struct to hold the `field/value` data for
/// the SocketLabs inline Merge feature
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Data {
    field: String,
    value: String,
}

/// This is a representation of a valid
/// SocketLabs email message
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    /// A vector of recipients for this message
    to: Vec<Email>,
    /// The sender for this message
    from: Email,
    /// The subject of this message
    subject: String,
    /// The text part of the message
    text_body: String,
    /// The optional html part of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    text_html: Option<String>,
    /// The optional integer ID referencing content
    /// from the Email Content Manager in the
    /// SocketLabs Control Panel
    #[serde(skip_serializing_if = "Option::is_none")]
    api_template: Option<String>,
    /// A SocketLabs header used to track batches of messages
    #[serde(skip_serializing_if = "Option::is_none")]
    mailing_id: Option<String>,
    /// A SocketLabs header used to tag individual messages
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<String>,
    /// The charset used for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    charset: Option<String>,
    /// Optional custom headers for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<Vec<CustomHeaders>>,
    /// A vector of recipients representing the
    /// cc'd recipients of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<Email>>,
    /// A vector of recipients representing the
    /// bcc'd recipients of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<Email>>,
    /// The email address to be used if replying to
    /// this email message
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<Email>,
    /// A vector of attached content blobs, such as images,
    /// documents and other binary files
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Vec<Attachment>>,
    /// Data storage for the inline Merge feature
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_data: Option<MergeData>,
}

impl Message {
    pub fn new(to: Vec<Email>, from: Email, subject: String, text_body: String) -> Message {
        Message {
            to: to,
            from: from,
            subject: subject,
            text_body: text_body,
            text_html: None,
            api_template: None,
            mailing_id: None,
            message_id: None,
            charset: None,
            custom_headers: None,
            cc: None,
            bcc: None,
            reply_to: None,
            attachment: None,
            merge_data: None,
        }
    }
}
