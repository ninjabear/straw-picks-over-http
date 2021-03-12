# straw-picks-over-http

![draw-straws](/draw-straws.jpg)

A small Rust/Rocket application to select a person from a list. Responses are cryptographically random and responses are digitally signed (with [ed25519](https://github.com/dalek-cryptography/ed25519-dalek)).

## Build + run

```{bash}
$ cargo build
$ cargo test
$ cargo run
```

## Endpoints

| Route     | Description                                        | 
|-----------|----------------------------------------------------| 
| `/`       | Public key                                         |
| `/?choices=A,B` | Pull a straw for `A` & `B`                   |