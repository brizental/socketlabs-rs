extern crate socketlabs;

use std::env;

use socketlabs::message::{Email, Message};
use socketlabs::request::Request;

fn main() {
    let message = Message::new(
        vec![Email::new("foo@bar.com".to_string(), None)],
        Email::new("bar@foo.com".to_string(), None),
        "test subject".to_string(),
        "baz".to_string(),
    );

    let request = Request::new(
        env::var("SOCKETLABS_SERVER_ID").unwrap().parse().unwrap(),
        env::var("SOCKETLABS_API_KEY").unwrap().to_string(),
        vec![message],
    ).unwrap();

    println!("{:?}", request.send().unwrap());
}
