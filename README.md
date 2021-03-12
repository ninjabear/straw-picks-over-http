# straw-picks-over-http

![draw-straws](/draw-straws.jpg)

A small Rust/Rocket application to select a person from a list. Responses (should be) cryptographically random* and responses are digitally signed (with [ed25519](https://github.com/dalek-cryptography/ed25519-dalek)).

\* it uses [rand::thread_rng](https://docs.rs/rand/0.6.5/rand/rngs/struct.ThreadRng.html) over [here](https://github.com/ninjabear/straw-picks-over-http/blob/main/src/picker.rs#L6).

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