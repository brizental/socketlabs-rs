# socketlabs-rs

Unofficial Rust library for the SocketLabs API.

socketlabs-rs is available on [crates.io](https://crates.io/crates/socketlabs) and can be included in your Cargo.toml as follows:

```toml
[dependencies]
socketlabs = "0.1.2"
```

## Running the example

You must have valid SocketLabs `server_id` and `api_key` to run the example.

```bash
SOCKETLABS_SERVER_ID=<your_server_id> SOCKETLABS_API_KEY=<your_api_key> cargo run --example send_email
```

## Generate docs

```bash
cargo doc --no-deps --lib --open
```

## License

This software is available under version 2.0 of the MPL:

https://www.mozilla.org/MPL/