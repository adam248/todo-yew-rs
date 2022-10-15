# Todo WebApp written in Rust using Yew.rs, with Trunk as the backend

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [Yew](https://yew.rs/)

## How to use

System-wide setup commands:

```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Project directory run command:

```
trunk serve
```

Trunk will compile the project and host the project live on port 8080.
