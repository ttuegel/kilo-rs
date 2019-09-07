# Build Your Own Text Editor in Rust

This is a "port", so to speak, of _Build Your Own Text Editor_ from C to Rust.
This guide describes the _differences_ between the C and Rust versions,
with links back to the original content.


## Setup

Rust and Cargo are used to build this project;
refer to [Install Rust](https://www.rust-lang.org/tools/install) for brief instructions.
The [Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html) gives a detailed introduction to the Rust compiler and Cargo.


## Entering raw mode

### read

The C version uses a `char` as a single-character buffer with `read()`
because C pointers do not distinguish between single values and arrays.
In contrast, Rust references _are_ type-safe,
so the Rust version uses a single-element array of `u8` as a buffer.
Note that Rust's `read_exact()` takes only the array as an argument;
we do not need to explicitly pass the length of the buffer
because it is reflected statically in the type!

### press-q

Rust's `char` is a Unicode character,
but we read a single, unsigned byte (`u8`) from the terminal,
so a conversion using `char::from()` is necessary.

### echo

Rather than depend on the C library, we use an external crate for `Termios`.
We use `as_raw_fd()` to get the file descriptor corresponding to the `io::stdin()` stream.

### atexit

Instead of using the C library's `atexit()` to register an exit handler,
we will use Rust's `Drop` trait to ensure that the terminal attributes are restored.
(Actually, the Rust runtime seems to do this for us,
but it is a good exercise anyway.)
This only seems to work if the program exits normally or panics;
the default signal handlers bypass this mechanism.

### keypresses

The significant difference between the Rust and C version
is that `println!()` and `printf()` expect different format strings;
actually, that's not so significant.
