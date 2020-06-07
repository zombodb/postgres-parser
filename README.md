[![Actions Status](https://github.com/zombodb/postgres-parser/workflows/cargo%20test%20--all/badge.svg)](https://github.com/zombodb/postgres-parser/actions)
[![crates.io badge](https://img.shields.io/crates/v/postgres-parser.svg)](https://crates.io/postgers-parser)
[![docs.rs badge](https://docs.rs/postgres-parser/badge.svg)](https://docs.rs/postges-parser)

[![Twitter Follow](https://img.shields.io/twitter/follow/zombodb.svg?style=social)](https://twitter.com/zombodb)  

## postgres-parser


This project is the beginnings of using Postgres' SQL Parser
(effectively `gram.y` and the `List *raw_parser(const char *str)` function)
from Rust.

The way this works is by downloading the Postgres source code, patching
a few of its Makefiles (see [`patches/makefiles-12.3.patch`](patches/makefiles-12.3.patch)),
compiling it to LLVM IR, optimizing/assembling that to LLVM bitcode, performing
link-time optimization (LTO) to generate a static library containing only the symbols/code
necessary to properly use Postgres' `raw_parser()` function, and finally, 
linking against that library with Rust.

This is accomplished via a custom [`build.rs`](build.rs) script, which 
shells out to [`build.sh`](build.sh) to perform all the hard work.

At the end of the process we're left with a `libpostgres_parser.a` archive, which
`build.rs` instructs cargo to link against.

### Using this Crate

Using this create is just like any other.  Add it as a dependency to your `Cargo.toml`:

```toml
[dependencies]
postgres-parser = "0.0.4"
```

Additionally, see the [System Requirements](#System+Requirements) section below.

Here's a simple example that outputs a SQL parse tree as JSON.

```rust
use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query_string = args.get(1).expect("no arguments");
    let parse_list = match parse_query(query_string) {
        Ok(query) => query,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    };

    let as_json = serde_json::to_string_pretty(&parse_list).expect("failed to convert to json");
    println!("{}", as_json);
}
```

### System Requirements

For an Ubuntu-based Linux system you'll need:

```shell script
$ sudo apt-get install clang llvm make curl
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For MacOS you'll need:

```shell script
$ brew install wget
$ brew install llvm
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

As Linux goes, so far this has been tested on Ubuntu 18.04 with LLVM 6.0.0, and
Ubuntu 20.04 with LLVM 10.0.0.

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
steps as the final `libpostgres_parser.a` archive artifact is cached in the `target/`
directory.


### Known Issues

- The parser currently only accepts SQL statements that adhere to what Postgres considers `SQL_ASCII` text.
The reason for this is that setting Postgres to support full `UTF8` (which is incredibly easy), somehow
causes the resulting compiled binary to bloat to nearly 10 megabytes.  This is being investigated

- Building on MacOS with XCode `>=11.4.0` doesn't work.  This appears to be a problem with these versions
of XCode.  This is the bug: https://openradar.appspot.com/FB7647406.  This happens while building Postgres.
Any suggestions for a work around would be greatly appreciated.

- Single-threaded query parsing... Postgres isn't thread safe, so we're required to lock on a Mutex while
parsing queries.  Which means one-at-a-time.  There may be some things we can do in the future to improve
this situation.  The underlying dilemma is around how Postgres allocates memory, and this approach to
embedding Postgres' parser necessitates it use that system

### Please Help!

We'd sincerely appreciate the time and effort you spend cloning this repo and at
least trying to `cargo test --all`.  If it doesn't work, or if these instructions are bad, 
we definitely want to know.  We'd like this to be as easy as possible for everyone.

Furthermore, this is v0.0.1.  Please feel free to submit bug reports, feature requests, and
most especially Pull Requests.

### Thanks

Thanks for checking this out.  [Here's the obligatory GitHub Sponsors link](https://github.com/sponsors/eeeebbbbrrrr).
  
If you like what we're doing and where this is going, your sponsorship will keep us 
motivated.

