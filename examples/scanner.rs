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
use postgres_parser::*;

/// A simple CLI example that parses the entire file specified by the first argument.
///
/// This example uses the `SqlStatementScanner` to scan the file and parse each full SQL
/// statement, one at a time.
///
/// It outputs each original SQL statement and its corresponding JSON representation.  If the
/// statement was a "COPY ... FROM stdin;" statement, then the subsequent data payload is
/// also output.
///
/// ## Usage
///
/// ```shell
/// $ target/debug/examples/scanner /path/to/schema.sql
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("no filename");
    let contents =
        std::fs::read_to_string(filename).expect(&format!("failed to read file: {}", filename));

    // the scanner is essentially an iterator that iterates over each complete SQL statement
    // in its input.  Here we construct a new SqlStatementScanner and iterate over the results
    let scanner = SqlStatementScanner::new(&contents);
    for (i, stmt) in scanner.into_iter().enumerate() {
        // output the raw SQL string from the scanner
        println!("#{}\n{}", i + 1, stmt.sql.trim_end());

        // and now we look at what it parsed
        match stmt.parsetree {
            // it was able, via Postgres' parser, to successfully parse the statement
            // output it as json
            Ok(parsetree) => {
                let as_json = serde_json::to_string(&parsetree).expect("failed to convert to json");
                println!("-- {}", as_json);

                // output corresponding payload data (likely COPY ... FROM stdin; data)
                if stmt.payload.is_some() {
                    println!("/* DATA:\n{}\n*/", stmt.payload.unwrap());
                }
            }

            // it couldn't be parsed.  Just display the underlying parse error
            Err(e) => {
                println!("-- ERROR:  {:?}", e);
            }
        };
        println!();
    }
}
