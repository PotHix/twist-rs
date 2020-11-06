Twist Java example
==================

To be able to use Rust from Java, one needs to export functions via JNI.  This
project imports the [JNI crate](https://crates.io/crates/jni) and uses its
types to expose the `search` function from twist-rs.

## Building

To build the project, [ensure you have rust and cargo
installed](https://www.rust-lang.org/tools/install) and also the latest version
of JDK.

The simplest version is to use `javac` on `Twist.java`:

        javac Twist.java

The `Twist.h` file should be generated already (for now we're keeping it in the
repo) but you can regenerate it by running:

        javac -h . Twist.java

The `Twist.h` file is important to link the Rust implementation to the Java one.

## Running

To run the project, you just have to execute:

        java Twist

If you find a `java.lang.UnsatisfiedLinkError` error, you have to be sure that
the library is in the right library path. On Linux you can:

        LD_LIBRARY_PATH=/path/to/twist-rs/target/debug java Twist
