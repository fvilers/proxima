# proxima

A lightweight HTTP server written in Rust

# Configuration

Proxima can be bound to the address and port of choice. There is different ways to configure this behavior. The priority order is environment variables, command line arguments, default values.

## Environment variables

Proxima will read the following environment variables

- ADDRESS
- PORT

## Command line arguments

Proxima accepts the following parameters:

| Name    | Short | Long      | Example    |
| ------- | ----- | --------- | ---------- |
| Address | -a    | --address | -a 0.0.0.0 |
| Port    | -p    | --port    | -p 8080    |

## Defaults

- Default address is `127.0.0.1`
- Default port is `80`

## Disclaimer

This hobby project only exists to help me learning and writing code in Rust. It may be production ready at some point, but without any guarantees.
