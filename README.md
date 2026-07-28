# Kangaroo 🦘

Kangaroos carry puppies in their pouches. `kangaroo` carries server-side data in static web applications, served by Axum.

This Rust crate allows you to serve static web applications and inject server-side data through Axum, making it available to the client as JSON objects. Kangaroo aims to avoid having heavy server runtimes serving apps through full SSR, but still be able to manipulate response status codes and reduce network round trips.

## Usage 🪿

> [!NOTE]
> **WIP**

## Development 👨‍💻

### Setup 🪛

- [Install Rust](https://www.rust-lang.org/tools/install)

### Run Locally 🧸

```sh
cargo run --example <example-name>
```

### Lint and Format 🧽

```sh
cargo fmt
cargo clippy -- --deny warnings
```
