/*
    Copyright (c) 2020, ZomboDB, LLC

    Permission to use, copy, modify, and distribute this software and its documentation for any purpose, without fee, and
    without a written agreement is hereby granted, provided that the above copyright notice and this paragraph and the
    following two paragraphs appear in all copies.

    IN NO EVENT SHALL ZomboDB, LLC BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL
    DAMAGES, INCLUDING LOST PROFITS, ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF ZomboDB, LLC
    HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

    ZomboDB, LLC SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
    MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS IS" BASIS, AND
    ZomboDB, LLC HAS NO OBLIGATIONS TO PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
*/
//! `postgres-parser` is a safe wrapper around Postgres' SQL query parser.  
//!
//! Its primary purpose is to easily syntax check SQL statements (individually or en masse) in
//! addition to providing a parse tree that can be walked, examined, mutated, or transformed.
//!
//! ## Technical Details
//!
//! First things first, this crate, as part of its build process, downloads the Postgres source
//! code, builds Postgres to LLVM IR (and then bitcode), which is ultimately statically linked into
//! the resulting Rust library (rlib), relying on LLVM's "link time optimization" (LTO) features to
//! reduce the Postgres LLVM bitcode to just the symbols/code required by this crate.
//!
//! That's a lot of work, and it requires that building this create, or even crates that use this
//! crate as a dependency, have the LLVM toolchain on the system `$PATH`.
//!
//! The justification for this is that, despite the build complexity, we can always stay current
//! with Postgres as it evolves its SQL support and, thus, its parser.
//!
//! ## What's in the Box?
//!
//! There's three primary things.  The first two are safe interfaces into parsing SQL statements
//! and evaluating the resulting parse trees.  The third is the set of completely unsafe functions
//! and structs upon which the first two are built.
//!
//! ### `parse_query()` and the `nodes` module
//!
//! The `parse_query()` function parses a string of SQL statements and returns a Vec of parsed
//! nodes, or a parse error
//!
//! A quick example:
//!
//! ```rust
//! use postgres_parser::{parse_query, PgParserError};
//! let parsetree = parse_query("SELECT * FROM my_table WHERE id = 42; SELECT 2;");
//! match parsetree {
//!     Ok(nodes) => {
//!         // one node for each statement parsed from the input string above
//!         for node in nodes {
//!             // debug-print the node for this query
//!             println!("{:#?}", node);
//!         }
//!     }
//!
//!     // one possible error, for the first mal-formed SQL statement
//!     Err(e) => {
//!         panic!(e);
//!     }      
//! }
//! ```
//!
//! The nodes represented in the parse tree live in the `postgres_parser::nodes` module.  The
//! top-level node is simply called `Node` and is an enum with a variant for every possible node
//! type.
//!
//! An example of walking a parsetree and examining the expected Node:
//!
//! ```rust
//! use postgres_parser::{parse_query, Node, join_name_list};
//! let parsetree = parse_query("DROP TABLE my_schema.my_table;");
//! match parsetree {
//!     Ok(mut nodes) => {
//!         let node = nodes.pop().unwrap(); // we know we only have 1 node here
//!         match node {
//!             Node::DropStmt(dropstmt) => {
//!                 // dropstmt.object is a Vec<Node>, where each Node is a Node::List of
//!                 // ultimately, Node::Value, where each value is a String
//!                 for object in dropstmt.objects.unwrap() {
//!                     // join_name_list() will figure out the hard part for us
//!                     // this is a common pattern throughout Postgres' parsetree
//!                     let name = join_name_list(&object).unwrap();
//!                     assert_eq!(name, "my_schema.my_table");
//!                 }
//!             }
//!
//!             _ => panic!("unexpected node: {:#?}", node),
//!         }
//!     }
//!
//!     // one possible error, for the first mal-formed SQL statement
//!     Err(e) => {
//!         panic!(e);
//!     }
//! }
//! ```
//!
//! ### The `sys` module
//!
//! The `sys` module is a 100% "bindgen"-generated module from Postgres' header files.  In general,
//! it's not expected that users of this crate will interact with this module.
//!
//! It is upon the items in this module that the rest of `postgres-parser` is built.  The module is
//! public for completeness only.
//!
//!
//! ### `SqlStatementScanner`
//!
//! The `SqlStatementScanner` is a simple type intended to work as an iterator over scanning and
//! parsing a single string of multiple SQL statements, one at a time.
//!
//! This is particullary useful to report statement-level parse errors, as opposed to the `parse_query()`
//! function that simply reports one error for the entire string.
//!
//! A quick example:
//!
//! ```rust
//! use postgres_parser::SqlStatementScanner;
//! let mut scanner = SqlStatementScanner::new("SELECT 1;\nSELECT 2;").into_iter();
//!
//! let first = scanner.next().expect("no first query");
//! assert_eq!(first.sql, "SELECT 1;\n"); // note trailing \n -- trailing whitespace after ';' is included
//! assert!(first.payload.is_none());
//! assert!(first.parsetree.is_ok());
//!
//! let second = scanner.next().expect("no second query");
//! assert_eq!(second.sql, "SELECT 2;");
//! assert!(second.payload.is_none());
//! assert!(second.parsetree.is_ok());
//!
//! assert!(scanner.next().is_none());
//! ```
//!
//! ## Serde Support
//!
//! All the parse tree Node structures supported are `Deserialize, Serialize`, and as such, can be
//! directly used by any of the serde serializers, including serde_json.
//!
//! ```rust
//!  use postgres_parser::parse_query;
//!  let as_json = serde_json::to_string_pretty(&parse_query("SELECT 1;")).expect("failed to convert to json");
//!  println!("{}", as_json);
//! ```
//!
//! The above would output:
//!
//! ```json
//! {"SelectStmt":{"targetList":[{"ResTarget":{"val":{"A_Const":{"val":{"int":1},"location":7}},"location":7}}],"op":"SETOP_NONE","all":false}}
//! ```
//!
//!
//! ## Notes on Thread Safety
//!
//! Postgres is, by design, not thread safe.  Rust, on the other hand, is.  As we're literally
//! statically linking against the compiled Postgres code, this presents an interesting problem.
//!
//! The solution `postgres-parser` has taken is that the `parse_query()` function (which is also
//! used by `SqlStatementScanner`) is guarded under a Rust Mutex.  As such, only one query can
//! be parsed at a time.
//!
use serde::{Deserialize, Serialize};

mod convert;
mod parser;
mod scanner;

pub mod nodes;
pub mod sys;

pub use nodes::Node;
pub use parser::*;
pub use scanner::*;

/// Represents various errors that can occur while parsing a SQL statement.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PgParserError {
    /// The SQL statement could not be parsed
    ParseError { message: String, cursor_pos: i32 },

    /// Internal SqlStatementScannerError -- more than one statement was scanned
    MultipleStatements(Vec<crate::Node>),

    /// We couldn't determine the error Postgres tried to tell us
    UnknownParseError,

    /// One of the returned parsed queries didn't parse to a `postgres-parser::RawStmt` (this is unlikely to ever happen)
    NotARawStmt,

    /// The input SQL statement `&str` contained an internal zero-byte
    InternalNull,

    /// The result from Postgres' parser was not a `Node::List` (this is unlikely to ever happen)
    NotAList,

    /// The evaluated Node is not a String
    NotAString,
}

/// A common pattern in Postgres' parse trees, when it needs to represent the name of a thing (a
/// table, a field, a view, etc), especially when that name can be qualified, is to represent
/// that name as a List of string Values.
///
/// This is a helper function to join these qualified name Lists together into a single Rust string.
///
/// It's expected that the `list_of_names` argument be a `postgres_parser::Node::List` variant,
/// where each of its contained members is a `postgres_parser::Node::Value` where its `string`
/// member `is_some()`.
///
/// If all these conditions are true, the result will be a dot-delimited string of each part of
/// the List of names (`Ok("the.qualified.name")`).
///
/// If not, the result will either be `Err(PgParserError::NotAList)` or
/// `Err(PgParserError::NotAString)`.
pub fn join_name_list(list_of_names: &crate::Node) -> Result<String, PgParserError> {
    match list_of_names {
        crate::Node::List(names) => {
            let mut result = String::new();
            for name in names {
                if !result.is_empty() {
                    result.push('.');
                }
                match name {
                    crate::Node::Value(value) if value.string.is_some() => {
                        result.push_str(value.string.as_ref().unwrap())
                    }
                    _ => return Err(PgParserError::NotAString),
                }
            }
            Ok(result)
        }

        _ => Err(PgParserError::NotAList),
    }
}
