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

* [ ] Implement one endpoint of the API as a library
* [ ] Expose this endpoint via CLI for easy manual testing and CLI exploration
* [ ] Make functions and structures more FFI-able
* [ ] Choose a secondary language and export this function using it


## Future goals

* [ ] Implement the happy path endpoints
* [ ] Upload as a crate to crates.io
* [ ] Create bindings for more languages if the multi-platform experiment succeeded


## Author

* PotHix <pothix at pothix dot com>
