Twist Ruby example
==================

To be able to use Rust from Java, one needs to connect to Rust via FFI.  This
project uses the [ffi ruby gem](https://rubygems.org/gems/ffi) to expose the
`search` function from twist-rs.

## Thoughts on this implementation

Using FFI from Ruby is quite simple but it comes with its own quirks. It's not
a problem of Ruby or Rust, it's just how FFI and C-like interfaces work.

Ruby is a dynamic language that has a GC, which is quite different from Rust
(or C), so we have to be careful on how we're dealing with the memory. The work
of taking care of the memory is a library maintainer's job.

If you created an object via Rust and passed this to Ruby, you have to take
care that both sides are aware of this. In other words, we're implementing the
ownership model manually, nothing much different than what C is doing since for
years.

The sentence above makes one think: "If we're doing C-style programming, why
using Rust then?". The answer to that is everything that is *not* in the
C-style boundary. There's also the other bindings to take into account, it's
not everything about FFI.

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
