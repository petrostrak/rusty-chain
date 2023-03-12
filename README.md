## Rusty chain!
A basic prototype and proof-of-work blockchain algorithm written in Rust!

### To play around
- Create wallet
```sh
cargo run create-wallet
```

- Create blockchain
```sh
cargo run create <address>
```

- Send coins (if `-m` is specified, the block will be mined immediately in the same node)
```sh
cargo run send <from> <to> <amount> -m
```