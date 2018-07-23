//!  Unofficial Rust library for the SocketLabs API. 
//!
//! The code for this project resides at [https://github.com/brizental/socketlabs-rs](https://github.com/brizental/socketlabs-rs)
//!
//! # Support
//!
//! The following APIs are **supported**
//!
//! * Injection
//!
//! The following APIs are **unsupported**
//!
//! * Notification
//! * Marketing
//! * Inbound
//! * Reporting
//! * On-Demand

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod message;
pub mod request;
pub mod response;
