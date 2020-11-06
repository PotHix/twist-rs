Twist Ruby example
==================

To be able to use Rust from Java, one needs to connect to Rust via FFI.  This
project uses the [ffi ruby gem](https://rubygems.org/gems/ffi) to expose the
`search` function from twist-rs.

## Building

To build the project, [ensure you have rust and cargo
installed](https://www.rust-lang.org/tools/install) and also the latest version
of Ruby and bundler.

This project uses the debug binary of the main crate to connect via FFI, so you
first need to compile it first.

Go to the root directory of this twist-rs project and run:

        cargo build

Now you can get back to this project and run:

        bundle install

It will install the needed gems and you should be good to go.

## Running

This project uses an environment variable called `auth` as the token to use for
Twist, so please set this environment variable first. If you are using a
unix-like OS, you can just `export` it:

        export auth=oauth2:10923847102983471029370127198743298

To run the project, you just have to execute:

        ruby rustffi.rb

_PS: not defining the environment variable will make the whole project to crash
by trying to read garbage from memory and trying to create a string 😅_
