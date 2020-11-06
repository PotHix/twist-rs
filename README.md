# twist-rs

This is a Rust implementation of a library for [Twist](http://twist.com), the
communication tool by [Doist](https://doist.com).

The main goal of this project is to serve as a proof of concept to check if
Rust is a good language to implement a multi-platform core library that can be
used by other languages to implement their own clients.

Given the multi-platform nature of Doist apps (kotlin/Java for Android,
swift/objective-c for iOS, Python for the backend, Javascript/typescript for
the frontend) and the urge to create libraries for the external developers,
it's worth exploring something that can deliver the core components of such a
library.

This project aims to deliver a simple CLI and start with a partial
implementation of a Twist library. The library will be used to validate the
multi-platform and multi-language support that Rust can provide.

## Initial goals

* [x] Implement one endpoint of the API as a library
* [x] Expose this endpoint via CLI for easy manual testing and CLI exploration
* [x] Make functions and structures more FFI-able
* [x] Choose a secondary language and export this function using it

## How to run it

It's a standard Rust crate and can be built with standard cargo commands:

        cargo build

This command will build the library and also a CLI so you can test it. The CLI
will be available at `./target/debug/twist`.

### Testing it manually via CLI

To manually test the CLI, you will need your Twist auth token defined an
environment variable called `auth`. If you are using a unix-like OS, you can
just `export` it:

        export auth=oauth2:10923847102983471029370127198743298

With the proper token defined, the CLI is ready.

There are few commands available now but you can try the search command:

        ./target/debug/twist search -q twist

For more information use the CLI help:

        ./target/debug/twist -h

## Author

* PotHix <pothix at pothix dot com>
