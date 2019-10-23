# Websocket PublicAPI - Pairs

[![Build Status](https://travis-ci.com/darkpools/darkpools_publicapi_ws_pairs.svg?branch=master)](https://travis-ci.com/darkpools/darkpools_publicapi_ws_pairs)

Copyright (C) 2019 Bitwyre Technologies LLC

## API Usages

This websocket service doesn't have TLS enabled nor cross-origin checking, it is suppose to be handled by an intermediary proxy.

Request & response example using [websocat](https://github.com/vi/websocat):

```json
$ websocat ws://localhost:4001/ws/public/pairs
get
{"pairs":["btc_usd","btc_idr","eth_btc"]}
```

Message anatomy:

- Structure name will always be "pairs"
- Result are tradeable currency pairs

## Development

### Pre-requisites

Rust-lang

```bash
curl https://sh.rustup.rs -sSf | sh
```

Development setup

```bash
./dev-setup.sh
```

### Configuration (via Environment)

- Maximum client count for this service to handle, new client(s) beyond this limit will be rejected during handshake
  - Environment Key is **MAX_CLIENT**
  - Default value is **16384**
- IP version 4 for this service to bind/listen to
  - Environment Key is **SERVICE_IP**
  - Default value is **0.0.0.0**
- The TCP port for this service to bind/listen to
  - Environment Key is **SERVICE_PORT**
  - Default value is **4001**
- Route path for the service
  - Environment Key is **SERVICE_PATH**
  - Default value is **/ws/public/pairs**
- Fastest interval between 2 ping originating from client, the client will get disconnected if too fast
  - Environment Key is **RAPID_REQUEST_LIMIT_MS**
  - Default value is **100**

### Running with script

Apply recommended code formatting

```text
./run.sh formatter
```

Tests

```text
./run.sh test
```

Lints

```text
./run.sh lint
```

Dependencies audit

```text
./run.sh audit
```

Development run

```text
./run.sh
```

## Contributors

- [Aditya Kresna](https://github.com/ujang360)
