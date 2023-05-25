# Problem
rkyv backward compatibility breaks with the new rust 1.69 release.


This example is meant to demonstrate the problem.

# Repro steps:
## 1. Run using rust 1.69
```
rustup default 1.69
cargo run
```

This should succeed.

## 2. Run using rust 1.68
```
rustup default 1.68
cargo run
```

This will fail

