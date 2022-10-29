# Semaphore

## Description

Crafted a server and client, using ipc-rpc of a simple semaphore.

## Usage

Build the client first:

```sh
cargo build -p client
```

Now, you can run the server, who calls the client by binary:

```sh
cargo run -p server -- "./crates/client/target/debug/client"
```

Done, you can see the output on the terminal.

### Notes

To run this project on `--release` version of the application workspace. Use
the following commands.

Build the client first:

```sh
cargo build -p client --release
```

Now, you can run the server, who calls the client by binary:

```sh
cargo run -p server -- "./crates/client/target/release/client" --release
```