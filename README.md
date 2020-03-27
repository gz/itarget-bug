```
# cargo --version
cargo 1.44.0-nightly (7019b3ed3 2020-03-17)

# $HOME/.cargo/bin/xargo  build  --target  x86_64-custom --color always -Zfeatures=itarget 
thread 'main' panicked at 'features did not find PackageId { name: "libc", version: "0.2.68", source: "registry `https://github.com/rust-lang/crates.io-index`" } false', src/tools/cargo/src/cargo/core/resolver/features.rs:220:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```