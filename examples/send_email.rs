extern crate socketlabs;

use std::collections::HashMap;
use std::env;

use socketlabs::message::Message;
use socketlabs::request::Request;

fn main() {
    let mut message = Message::new("foo@bar.com", None);
    message.add_to("bar@foo.com", None);
    message.set_subject("Hello from the socketlabs-rs example");
    message.set_text("Hello, text world!");
    message.set_html("<p><strong>Hello, HMTL world!</strong></p>");
    let mut headers = HashMap::new();
    headers.insert("x-example", "hey hey hey");
    message.add_headers(headers);

    let request = Request::new(
        env::var("SOCKETLABS_SERVER_ID").unwrap().parse().unwrap(),
        env::var("SOCKETLABS_API_KEY").unwrap().to_string(),
        vec![message],
    ).unwrap();

    println!("{:#?}", request.send().unwrap());
}
