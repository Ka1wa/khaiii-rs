# khaiii-rs
[Documentation](https://docs.rs/khaiii-rs)

[khaiii](https://github.com/kakao/khaiii) bindings for Rust.

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

## Usage
Using khaiii-rs is easiest with khaiii pre-installed on the system. 

In the `examples/` folder are two Rust files demonstrating how to initialize the khaiii API and how to analyze Korean text.

Critically, before the API can analyze text it has to load a couple of resources. On a pre-installed system these can be found under the default `/usr/local/share/khaiii` directory. On systems without a global installation, the resource files can be found in the `khaiii-rs/share/khaiii` folder after performing a cargo build with the vendored-khaiii feature enabled as previously shown in the building section of this README.

## Operating systems
As of right now khaiii-sys, (and therefore khaiii-rs), is only developed for and tested on Linux. While, like khaiii itself, it might work on MacOS (non-ARM) I am unfortunately unable to test and support it.

# License
This project is licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or
  https://www.apache.org/licenses/LICENSE-2.0)
