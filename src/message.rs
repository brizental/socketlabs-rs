use std::collections::HashMap;
use std::hash::Hash;

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
    custom_headers: Option<Vec<CustomHeader>>,
}

/// This is a representation of email headers
/// that corresponds to the way SocketLabs represents them
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CustomHeader {
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
    html_body: Option<String>,
    /// The optional integer ID referencing content
    /// from the Email Content Manager in the
    /// SocketLabs Control Panel More about it:
    /// http://www.socketlabs.com/blog/introducing-api-templates/
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
    custom_headers: Option<Vec<CustomHeader>>,
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
    pub fn new<T: Into<String>>(address: T, name: Option<T>) -> Message {
        let from = match name {
            Some(name) => Email::new(address.into(), Some(name.into())),
            None => Email::new(address.into(), None),
        };

        Message {
            to: Vec::new(),
            from: from,
            subject: String::new(),
            text_body: String::new(),
            html_body: None,
            api_template: None,
            mailing_id: None,
            message_id: None,
            charset: None,
            custom_headers: None,
            cc: None,
            bcc: None,
            reply_to: None,
            // TODO: create add_attachment function
            attachment: None,
            // TODO: create add_merge_data function
            merge_data: None,
        }
    }

    /// Adds a new recipient to the Message struct.
    pub fn add_to<T: Into<String>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.to.push(Email::new(address.into(), Some(name.into()))),
            None => self.to.push(Email::new(address.into(), None)),
        }
    }

    /// Sets the from field in the Message struct.
    pub fn set_from<T: Into<String>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.from = Email::new(address.into(), Some(name.into())),
            None => self.from = Email::new(address.into(), None),
        }
    }

    /// Sets the subject field in the Message struct.
    pub fn set_subject<T: Into<String>>(&mut self, subject: T) {
        self.subject = subject.into()
    }

    /// Sets the text_body field in the Message struct.
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text_body = text.into()
    }

    /// Sets the html_body field in the Message struct.
    pub fn set_html<T: Into<String>>(&mut self, html: T) {
        self.html_body = Some(html.into())
    }

    /// Sets the api_template field in the Message struct.
    pub fn set_api_template<T: Into<String>>(&mut self, api_template: T) {
        self.api_template = Some(api_template.into())
    }

    /// Sets the message_id field in the Message struct.
    pub fn set_message_id<T: Into<String>>(&mut self, message_id: T) {
        self.message_id = Some(message_id.into())
    }

    /// Sets the charset field in the Message struct.
    pub fn set_charset<T: Into<String>>(&mut self, charset: T) {
        self.charset = Some(charset.into())
    }

    /// Adds headers to the custom_header field in the Message struct
    pub fn add_headers<T: Into<String> + Eq + Hash>(&mut self, headers: HashMap<T, T>) {
        if self.custom_headers.is_none() {
            self.custom_headers = Some(Vec::new());
        }

        if let Some(ref mut custom_headers) = self.custom_headers {
            for (name, value) in headers {
                custom_headers.push(CustomHeader {
                    name: name.into(),
                    value: value.into(),
                })
            }
        } 
    }

    /// Adds a new cc'd recipient to the Message struct.
    pub fn add_cc<T: Into<String>>(&mut self, address: T, name: Option<T>) {
        let email = match name {
            Some(name) => Email::new(address.into(), Some(name.into())),
            None => Email::new(address.into(), None),
        };

        match self.cc {
            Some(ref mut cc) => cc.push(email),
            None => self.cc = Some(vec![email]),
        }
    }

    /// Adds a new bcc'd recipient to the Message struct.
    pub fn add_bcc<T: Into<String>>(&mut self, address: T, name: Option<T>) {
        let email = match name {
            Some(name) => Email::new(address.into(), Some(name.into())),
            None => Email::new(address.into(), None),
        };

        match self.bcc {
            Some(ref mut bcc) => bcc.push(email),
            None => self.bcc = Some(vec![email]),
        }
    }

    /// Sets the from field in the Message struct.
    pub fn set_reply_to<T: Into<String>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.reply_to = Some(Email::new(address.into(), Some(name.into()))),
            None => self.reply_to = Some(Email::new(address.into(), None)),
        }
    }
}
