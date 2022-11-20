# khaiii-rs
[Documentation](https://docs.rs/khaiii-rs)

khaiii bindings for Rust.

```toml
[dependencies]
khaiii-rs = "0.1"
```

## Requirements
The following software is required to use khaiii-rs:
* CMake 3.10 or up
* Python3
* Stable Rust. (khaiii-rs was developed with Rust 1.65.0 but should work with most stable Rust versions)

## Version of khaiii
Currently this library requires khaiii 0.4. The source for khaiii is
included within the khaiii-sys crate. If khaiii is not already pre-installed on your system you can use the `vendored-khaiii` feature flag so that the build script will compile, link and generate the khaiii resources for you instead.

## Building khaiii-rs
Systems with khaiii pre-installed:
```sh
$ git clone https://github.com/ka1wa/khaiii-rs
$ cd khaiii-rs
$ cargo build
```

Build with vendored source:
```sh
$ git clone https://github.com/ka1wa/khaiii-rs
$ cd khaiii-rs
$ cargo build -F vendored-khaiii
```

## Testing
Simple tests for the khaiii-rs wrapper are included and can be run through the following command. Similarly to the build command with khaiii pre-installed you can simply run:
```sh
$ cargo test
```

Testing with vendored source: 
```sh
$ cargo test -F vendored-khaiii
```

## Operating systems
As of right now khaiii-sys, (and therefore khaiii-rs), only supports Linux.

# License
This project is licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or
  https://www.apache.org/licenses/LICENSE-2.0)
