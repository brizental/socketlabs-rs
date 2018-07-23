/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A representation of a valid email
//! message for SocketLabs [Injection API](https://www.socketlabs.com/api-reference/injection-api/).

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

/// This is a representation of email attachments
/// that corresponds to the way SocketLabs represents them.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Attachment<'a> {
    /// The name of the attachment
    name: Cow<'a, str>,
    /// A description of the content in the attachment
    content: Cow<'a, str>,
    /// The id of the content in the attachment
    content_id: Cow<'a, str>,
    /// The type of the content in the attachment
    content_type: Cow<'a, str>,
    /// The headers in the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<Vec<CustomHeader<'a>>>,
}

/// This is a representation of email headers
/// that corresponds to the way SocketLabs represents them.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CustomHeader<'a> {
    /// The name of the header
    name: Cow<'a, str>,
    /// The value of the header
    value: Cow<'a, str>,
}

/// This is a representation of an email address
/// plus the optional name of the owner of that address.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Email<'a> {
    /// The actual email address
    email_address: Cow<'a, str>,
    /// The name of the owner of the address
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<Cow<'a, str>>,
}

impl<'a> Email<'a> {
    pub fn new(email_address: Cow<'a, str>, friendly_name: Option<Cow<'a, str>>) -> Email<'a> {
        Email {
            email_address: email_address,
            friendly_name: friendly_name,
        }
    }
}

/// This is a representation of the data storage for the
/// inline Merge feature from SocketLabs. More about it:
/// [https://www.socketlabs.com/blog/unleash-power-merge-fields/].
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MergeData<'a> {
    /// A vector used to define merge field data for each
    /// message. Variables can be freely named, with the
    /// exception of a single reserved word, `DeliveryAddress`
    /// which defines the recipient of the current message
    per_message: Vec<Data<'a>>,
    /// A vector used to define merge field data for all
    /// messages in the injection
    global: Vec<Data<'a>>,
}

/// Helper struct to hold the `field/value` data for
/// the SocketLabs inline Merge feature.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Data<'a> {
    field: Cow<'a, str>,
    value: Cow<'a, str>,
}

/// This is a representation of a valid
/// SocketLabs email message.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message<'a> {
    /// A vector of recipients for this message.
    to: Vec<Email<'a>>,
    /// The sender for this message.
    from: Email<'a>,
    /// The subject of this message.
    subject: Cow<'a, str>,
    /// The text part of the message.
    text_body: Cow<'a, str>,
    /// The optional html part of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    html_body: Option<Cow<'a, str>>,
    /// The optional integer ID referencing content
    /// from the Email Content Manager in the
    /// SocketLabs Control Panel More about it:
    /// http://www.socketlabs.com/blog/introducing-api-templates/.
    #[serde(skip_serializing_if = "Option::is_none")]
    api_template: Option<Cow<'a, str>>,
    /// A SocketLabs header used to track batches of messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    mailing_id: Option<Cow<'a, str>>,
    /// A SocketLabs header used to tag individual messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Cow<'a, str>>,
    /// The charset used for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    charset: Option<Cow<'a, str>>,
    /// Optional custom headers for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<Vec<CustomHeader<'a>>>,
    /// A vector of recipients representing the
    /// cc'd recipients of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<Email<'a>>>,
    /// A vector of recipients representing the
    /// bcc'd recipients of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<Email<'a>>>,
    /// The email address to be used if replying to
    /// this email message.
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<Email<'a>>,
    /// A vector of attached content blobs, such as images,
    /// documents and other binary files.
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Vec<Attachment<'a>>>,
    /// Data storage for the inline Merge feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_data: Option<MergeData<'a>>,
}

impl<'a> Message<'a> {
    /// Create a new Message object with all fields empty
    /// but the `from` field.
    pub fn new<T: Into<Cow<'a, str>>>(address: T, name: Option<T>) -> Message<'a> {
        let from = match name {
            Some(name) => Email::new(address.into(), Some(name.into())),
            None => Email::new(address.into(), None),
        };

        Message {
            to: Vec::new(),
            from: from,
            subject: "".into(),
            text_body: "".into(),
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
    pub fn add_to<T: Into<Cow<'a, str>>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.to.push(Email::new(address.into(), Some(name.into()))),
            None => self.to.push(Email::new(address.into(), None)),
        }
    }

    /// Sets the from field in the Message struct.
    pub fn set_from<T: Into<Cow<'a, str>>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.from = Email::new(address.into(), Some(name.into())),
            None => self.from = Email::new(address.into(), None),
        }
    }

    /// Sets the subject field in the Message struct.
    pub fn set_subject<T: Into<Cow<'a, str>>>(&mut self, subject: T) {
        self.subject = subject.into()
    }

    /// Sets the text_body field in the Message struct.
    pub fn set_text<T: Into<Cow<'a, str>>>(&mut self, text: T) {
        self.text_body = text.into()
    }

    /// Sets the html_body field in the Message struct.
    pub fn set_html<T: Into<Cow<'a, str>>>(&mut self, html: T) {
        self.html_body = Some(html.into())
    }

    /// Sets the api_template field in the Message struct.
    pub fn set_api_template<T: Into<Cow<'a, str>>>(&mut self, api_template: T) {
        self.api_template = Some(api_template.into())
    }

    /// Sets the message_id field in the Message struct.
    pub fn set_message_id<T: Into<Cow<'a, str>>>(&mut self, message_id: T) {
        self.message_id = Some(message_id.into())
    }

    /// Sets the charset field in the Message struct.
    pub fn set_charset<T: Into<Cow<'a, str>>>(&mut self, charset: T) {
        self.charset = Some(charset.into())
    }

    /// Adds headers to the custom_header field in the Message struct.
    pub fn add_headers<T: Into<Cow<'a, str>> + Eq + Hash>(&mut self, headers: HashMap<T, T>) {
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
    pub fn add_cc<T: Into<Cow<'a, str>>>(&mut self, address: T, name: Option<T>) {
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
    pub fn add_bcc<T: Into<Cow<'a, str>>>(&mut self, address: T, name: Option<T>) {
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
    pub fn set_reply_to<T: Into<Cow<'a, str>>>(&mut self, address: T, name: Option<T>) {
        match name {
            Some(name) => self.reply_to = Some(Email::new(address.into(), Some(name.into()))),
            None => self.reply_to = Some(Email::new(address.into(), None)),
        }
    }
}
