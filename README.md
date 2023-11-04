Compiler Crash
==============

Minimal example to reproduce several compiler crashes that I've encountered.
See rustc [issue #117460](https://github.com/rust-lang/rust/issues/117460).

Build each crash separatly like this:

```bash
cargo build --features=crash1
cargo build --features=crash2
cargo build --features=crash3
cargo build --features=crash4
cargo build --features=crash5
cargo build --features=crash6
```

Or use the helper script that generates `rustc-ice-crash$i.txt` and
`rustc-out-crash$i.txt` files:

```bash
./build_all.sh
```

The `cargo version` with which the crash files in this repository where created:

```plain
cargo 1.75.0-nightly (b4d18d4bd 2023-10-31)
```
