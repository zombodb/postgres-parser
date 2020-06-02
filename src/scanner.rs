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
use crate::{parse_query, Node, PgParserError};
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::str::CharIndices;

/// An individual SQL statement scanned from a larger set of SQL statements
///
/// Includes the raw SQL, its parsing result, and potentially an attached payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScannedStatement<'a> {
    /// The raw SQL statement
    pub sql: &'a str,

    /// Parsing results
    pub parsetree: std::result::Result<Option<Node>, PgParserError>,

    /// If the parsed statement resulted in a "COPY ... FROM stdin;" statement,
    /// this will contain the copy data that follows
    pub payload: Option<&'a str>,
}

/// The `SqlStatementScanner` allows for scanning a blob of SQL statements and ultimately
/// iterating over each statement, one at a time, producing a `ScannedStatement` that includes
/// the raw SQL, that SQL's parsetree, and optional "COPY ... FROM stdin;" payload data.
///
/// When parsing multiple SQL statements, the `SqlStatementScanner` is superior to directly calling
/// `parse_query()` as each individual statement is parsed on its own.  This provides for syntax
/// checking each statement individually, instead of failing if any statement is syntactically
/// incorrect.
///
/// ## Statement Scanning Notes
///
/// - Trailing whitespace after a statement end (`"SELECT 1;  \n\n    SELECT 2;"`) are included with
/// the preceding statement.  As such, this would scan into two statements: `["SELECT 1;  \n\n    ", "SELECT 2;"]`
/// - Statement-terminating semicolons are included with the statement
/// - The final statement need not have a terminating semicolon
///
///
/// ## Examples
///
/// Parsing multiple statements:
///
/// ```rust
/// use postgres_parser::SqlStatementScanner;
/// let scanner = SqlStatementScanner::new("SELECT 0; SELECT 1; SELECT 2;");
/// for (idx, statement) in scanner.iter().enumerate() {
///     assert_eq!(statement.sql.trim_end(), &format!("SELECT {};", idx));
/// }
/// ```
///
pub struct SqlStatementScanner<'a> {
    sql: &'a str,
}

impl<'a> SqlStatementScanner<'a> {
    pub fn new(sql: &'a str) -> Self {
        SqlStatementScanner { sql }
    }

    pub fn iter(&self) -> SqlStatementScannerIterator {
        SqlStatementScannerIterator {
            sql: self.sql,
            start: 0,
        }
    }
}

impl<'a> IntoIterator for SqlStatementScanner<'a> {
    type Item = ScannedStatement<'a>;
    type IntoIter = SqlStatementScannerIterator<'a>;

    fn into_iter(self) -> SqlStatementScannerIterator<'a> {
        SqlStatementScannerIterator {
            sql: self.sql,
            start: 0,
        }
    }
}

/// Iterator for the `SqlStatementScanner`
pub struct SqlStatementScannerIterator<'a> {
    sql: &'a str,
    start: usize,
}

impl<'a> Iterator for SqlStatementScannerIterator<'a> {
    type Item = ScannedStatement<'a>;

    fn next(&mut self) -> Option<ScannedStatement<'a>> {
        self.scan_statement()
    }
}

impl<'a> SqlStatementScannerIterator<'a> {
    fn scan_statement(&mut self) -> Option<ScannedStatement<'a>> {
        if self.start >= self.sql.len() {
            return None;
        }

        let mut sql = None;

        let mut in_sl_comment = false;
        let mut in_ml_comment = false;
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut current_dollar_quote = None;

        let input = &self.sql[self.start..];
        let mut iter = input.char_indices().peekable();
        let mut putback = None;

        fn get_next(
            putback: &mut Option<(usize, char)>,
            iter: &mut Peekable<CharIndices>,
        ) -> Option<(usize, char)> {
            if putback.is_some() {
                // if we have a pushback item, take and return that
                putback.take()
            } else {
                // otherwise just return the next item in the iterator
                iter.next()
            }
        }

        while let Some((mut idx, c)) = get_next(&mut putback, &mut iter) {
            let mut nextc = match iter.peek() {
                Some((_, c)) => *c,
                None => 0 as char,
            };

            match c {
                // toggle us in/out of dollar quotes
                '$' => {
                    if !(in_sl_comment || in_ml_comment) {
                        let begin = idx;
                        let mut end = idx + 1;

                        // scan ahead for the ending quote
                        let mut incomplete = false;

                        loop {
                            match iter.next() {
                                Some((idx, c)) => {
                                    end = idx;
                                    if c == '$' {
                                        break;
                                    } else if !c.is_alphanumeric() && c != '_' {
                                        // it's not a valid dollar quote
                                        // we save this value and push it back
                                        // so we can process it as it is
                                        putback = Some((idx, c));
                                        break;
                                    }
                                }
                                None => {
                                    incomplete = true;
                                    break;
                                }
                            }
                        }

                        if putback.is_some() {
                            continue;
                        }

                        if !incomplete {
                            // we found a complete $dollar_quote$
                            let quote = &input[begin..=end];

                            match current_dollar_quote.as_ref() {
                                Some(current) => {
                                    if quote == *current {
                                        // and it matches the one we're looking for
                                        // which means we're actually ending the quote block
                                        current_dollar_quote = None;
                                    }
                                }
                                None => {
                                    // we don't have a dollar quotes yet, so we must
                                    // have just started one
                                    current_dollar_quote = Some(quote);
                                }
                            }
                        }
                    }
                }

                // toggle us in/out of double quotes
                '"' => {
                    if !(in_sl_comment || in_ml_comment) {
                        in_double_quote = !in_double_quote;
                    }
                }

                // toggle us in/out of single quotes
                '\'' => {
                    if !(in_sl_comment || in_ml_comment) {
                        in_single_quote = !in_single_quote;
                    }
                }

                // skip over escaped characters
                '\\' => {
                    if !(in_sl_comment || in_ml_comment) {
                        iter.next();
                    }
                }

                // we're in a C-style single_line comment
                '/' if nextc == '/' => {
                    if !in_ml_comment {
                        in_sl_comment = true;
                    }
                }

                // we're in an SQL-style single_line comment
                '-' if nextc == '-' => {
                    if !in_ml_comment {
                        in_sl_comment = true;
                    }
                }

                // no longer in a single_line comment
                '\r' | '\n' => {
                    in_sl_comment = false;
                }

                // we're in a multi_line comment
                '/' if nextc == '*' => {
                    if !in_sl_comment {
                        in_ml_comment = true;
                    }
                }

                // no longer in a multi_line comment
                '*' if nextc == '/' => {
                    if !in_sl_comment {
                        in_ml_comment = false;
                    }
                }

                // end of a statement
                ';' => {
                    if !(in_sl_comment || in_ml_comment)
                        && !(in_single_quote || in_double_quote || current_dollar_quote.is_some())
                    {
                        // include trailing whitespace in this statement
                        while nextc.is_whitespace() && iter.next().is_some() {
                            nextc = match iter.peek() {
                                Some((_, c)) => {
                                    idx += 1;
                                    *c
                                }
                                None => 0 as char,
                            };
                        }

                        sql = Some(&input[..=idx]);

                        // this is where the next statement will start, if there is one
                        self.start += idx + 1;

                        break;
                    }
                }

                // otherwise we don't care about the character
                _ => {}
            }
        }

        if sql.is_none() {
            if self.start < self.sql.trim_end().len() {
                // we have a trailing statement that didn't end with a semicolon
                sql = Some(input);

                // and we're done here
                self.start += input.len();
            } else {
                return None;
            }
        }

        let sql = sql.unwrap();
        let (parsed, payload) = match parse_query(sql) {
            // if it only has one Node (which most will if the scanner worked correctly!), it
            // might be a "CopyStmt" node, and if it is we need to gather up its data payload
            Ok(mut vec) if vec.len() == 1 => {
                let stmt = vec.get(0).unwrap();
                let payload = match stmt {
                    Node::CopyStmt(copy) => {
                        if copy.is_from == true
                            && copy.is_program == false
                            && copy.filename.is_none()
                        {
                            // it's a "COPY table FROM stdin;" statement, so it should
                            // have a payload that follows
                            self.scan_copy_data()
                        } else {
                            // it's not the COPY statement we're looking for
                            None
                        }
                    }
                    _ => None,
                };

                (Ok(Some(vec.remove(0))), payload)
            }

            // no Nodes were parsed.  Query was likely just whitespace or a single ";"
            // this is still an Ok() situation
            Ok(vec) if vec.len() == 0 => (Ok(None), None),

            // more than one Node was parsed -- this shouldn't happen if the scanner works properly!
            Ok(vec) => (Err(PgParserError::MultipleStatements(vec)), None),

            // parsing failed
            Err(e) => (Err(e), None),
        };

        Some(ScannedStatement {
            sql,
            parsetree: parsed,
            payload,
        })
    }

    fn scan_copy_data(&mut self) -> Option<&'a str> {
        let input = &self.sql[self.start..];

        let mut prevc = '\n' as char;
        let mut iter = input.char_indices().peekable();
        while let Some((idx, c)) = iter.next() {
            let nextc = match iter.peek() {
                Some((_, c)) => *c,
                None => 0 as char,
            };

            match c {
                // found the terminator
                '\\' if nextc == '.' && prevc == '\n' => {
                    self.start += idx + 2; // +2 to skip the '.' for the next statement scan

                    return Some(&input[..=idx + 1]); // +1 to include the '.' in the copy data
                }
                _ => {}
            }

            prevc = c;
        }

        // no matching copy data found
        None
    }
}
