# GRPC-SUI

### Author
Jan Rock (rock@linux.com)

### Version
0.1.0 (5/3/2024)

## Description
Simple Rust gRPC backend application for interation with SUI Move smart contract.

## Install

## Compile

### Smart Contract

From contract folder:
```shell
sui client envs
sui client new-env --alias devnet --rpc https://fullnode.devnet.sui.io:443
sui client active-env
sui client switch --env devnet
sui client active-env
sui client publish --gas-budget 200000000 .
```

## Run

### Server

Environment variables for logging and MongoDB:
```shell
export RUST_LOG=info # or debug
export MONGODB_URL="mongodb+srv://<user>:<password>@cluster0.gsjcz.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"
export MONGODB_DATABASE="<database>"
```

Run with Cargo:
```shell
cargo run --bin grpc_server
```

### Client

```shell
export RUST_LOG=info # or debug
```

Run with Cargo:
```shell
cargo run --bin grpc_client
```

## Release

```shell
cargo build --release

./target/release/grpc_server
# or
./target/release/grpc_client
```