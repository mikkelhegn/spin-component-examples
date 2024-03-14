# sqlite-proxy

This repo is an example of how to use a proxy component for the wasi-sqlite interface, to intercept calls to a sqlite database in the Spin World.

## Repo structure

The `proxy/` directory contains a proxy to intercept executions against a sqlite database, and obfuscate e-mail addresses returned from the database. The proxy is written in Rust.

The `example/` directory contains a Spin application which consists of four http handlers which returns data from a sqlite database in the body.

- no-proxy-rust
  - A component written in Rust, returning unfiltered data from the database.
- proxy-rust
  - A component composition of the no-proxy-rust component and the proxy. This component returns data with obfuscated e-mails.
- no-proxy-python
  - A component written in Python, returning unfiltered data from the database.
- proxy-python
  - A component composition of the no-proxy-python component and the proxy. This component returns data with obfuscated e-mails.

## Demo instructions

### Pre-requisites

- Install [cargo component](https://github.com/bytecodealliance/cargo-component):

```bash
cargo install --git https://github.com/bytecodealliance/cargo-component cargo-component
```

- Install [wasm-tools](https://github.com/bytecodealliance/wasm-tools)

```bash
cargo install wasm-tools
```

- Install [Spin](https://developer.fermyon.com/spin/v2/install)

### Build the components and run the demo

```bash
cd example

# Build the Spin application and proxy. Spin build runs all the build commands to build the four components. Check out the commands in the `spin.toml` file.
spin build

# Build and run the example
spin up --sqlite @db.sql

# Curl the four endpoints in a browser
curl -i http://127.0.0.1:3000/no-proxy-rust
curl -i http://127.0.0.1:3000/proxy-rust
curl -i http://127.0.0.1:3000/no-proxy-python
curl -i http://127.0.0.1:3000/proxy-python
```

Now deploy your application.

```sh
$ spin deploy
Uploading sqlite_proxy_example version 0.1.0 to Fermyon Cloud...
Deploying...
Waiting for application to become ready............. ready
Available Routes:
  example: https://sqlite_proxy_example-12345.fermyon.app (wildcard)
```

In the example deploy output above, the app now exists at endpoint `https://example-12345.fermyon.app`.
