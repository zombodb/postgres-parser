## postgres-parser

Currently, this project is not remotely ready for use of any kind!

This project is the beginnings of using Postgres' SQL Parser
(effectively `gram.y` and the `List *raw_parser(const char *str)` function)
from Rust.

The way this works is by downloading the Postgres source code, patching
a few of its Makefiles (see [`patches/makefiles-12.3.patch`](patches/makefiles-12.3.patch)),
compiling it to LLVM IR, converting that to LLVM bitcode, and
linking against it with Rust, using LTO (link-time-optimization) to ensure 
that the resulting Rust library only contains the bits needed parse SQL 
statements, and not the entirety of Postgres.

This is accomplished via a custom [`build.rs`](build.rs) program, which 
shells out to [`build.sh`](build.sh) to perform all the hard work.

At the end of the process we're left with a `libpostgres.a` archive, which
`build.rs` instructs cargo to link against.

There's a few `RUSTFLAGS` set in [`.cargo/config`](.cargo/config) which are
necessary to tell Rust which linker we need to use (we don't want to mix/match
`clang` and `gcc` -- we only want `clang`!) along with the LTO flags.

### About & TODO

Right now, this project simply produces a dumb CLI binary that attempts to
parse its first argument as an SQL query.

Future (very near future!) updates will convert it into a Rust library (instead 
of the binary it currently is) and provide proper Rust wrappers around Postgres' 
parser, syntax error handling, along with conversion of all Postgres' 
various parse node structs.

There will also be `serde` support for serializing parse trees to whatever serde
format you want.

### System Requirements

For an Ubuntu-based Linux system you'll need:

```shell script
$ sudo apt get install clang
$ sudo apt get install llvm
$ sudo apt get install make
$ sudo apt get install curl
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For MacOS you'll need:

```shell script
$ brew install wget
$ brew install llvm
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

In both environments, you want to ensure you're installing LLVM (and clang) 
version 10.0.0.  It's possible that earlier versions will also work, but
this has not been tested.

You'll also want to make sure the LLVM and clang tools are on your `$PATH`.
Especially the `clang`, `opt`, and `llvm-ar` tools.

### Building

Build this just like any other Rust binary crate:

```shell script
$ cargo build [--release]
```

This will take awhile as again, the build process:

    - Downloads Postgres source code
    - Configures Postgres
    - Compiles Postgres (in parallel up to # of your CPUS)
    - Optimizes the resulting LLVM ir into LLVM bitcode

On my relatively new MacBook Pro 16", this process takes about 2.5 minutes the first
time.  

On my incredibly old Mac Mini, running Ubuntu 16.04 (yikes!), this process takes about
25 minutes.  So be patient if you have an older computer.

Subsequent builds (assuming no `cargo clean`) are able to elide all of the above 
steps as the final `libpostgres.a` archive artifact is cached in the `target/`
directory.

### Please Help!

I'd sincerely appreciate the time and effort you spend cloning this repo and at
least trying to `cargo build` it on your machine.  If it doesn't work, or if my
instructions are bad, I definitely want to know.  I'd like this to be as easy as
possible for everyone.

In fact, it'd be super cool if you simply tried to `cargo build` it without first
ensuring you're on LLVM 10.0.0 -- it may be that LLVM 10.0.0 isn't required.

### Thanks

Thanks for checking this out.  [Here's the obligatory GitHub Sponsors link](https://github.com/sponsors/eeeebbbbrrrr).
  
If you like what I'm doing and where this is going, your sponsorship will keep me 
motivated.

