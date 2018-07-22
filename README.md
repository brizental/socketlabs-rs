# socketlabs-rs

![Under construction](https://camo.githubusercontent.com/45d551b3b690a49aa6d855f9fe28fd47a5effc82/68747470733a2f2f63646e2e74686561746c616e7469632e636f6d2f6173736574732f6d656469612f696d672f706f7374732f323031352f31302f6d616d61676e6f6c69615f6163726573756e646572636f6e737472756374696f6e2f6132613838353234352e676966)

Unofficial Rust library for the SocketLabs API.

### Running the example

You must have valid SocketLabs `server_id` and `api_key` to run the example.

```bash
SOCKETLABS_SERVER_ID=<your_server_id> SOCKETLABS_API_KEY=<your_api_key> cargo run --example send_email
```

### Generate docs

```bash
cargo doc --no-deps --lib --open
```