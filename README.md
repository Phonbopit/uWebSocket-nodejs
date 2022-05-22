# uWebSocket-nodejs

uWebSocket with Node.js

## Installation

```
yarn add uWebSockets.js@uNetworking/uWebSockets.js#v20.10.0
```

## Start server (Node.js)

```
yarn start:server

# Or node app.js
```

## Start client (Rust)

```
cd rs_ws_client

cargo build --release
RUST_LOG=debug ./target/release/rs_ws_client

# Or with yarn script
yarn build
yarn start:client
```


> You can choose any []release version](https://github.com/uNetworking/uWebSockets.js/releases)
