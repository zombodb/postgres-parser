use crate::{parse_query, Node, PgParserError};
use serde::{Deserialize, Serialize};
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannedStatement<'a> {
    pub sql: &'a str,
    pub parsed: std::result::Result<Vec<Node>, PgParserError>,
    pub payload: Option<&'a str>,
}

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
        let mut iter = input.chars().enumerate().peekable();
        let mut putback = None;

        fn get_next(
            putback: &mut Option<(usize, char)>,
            iter: &mut Peekable<Enumerate<Chars>>,
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
            // might be a "CopyStmt" node, and if it is we need to
            // gather up its data payload
            Ok(vec) if vec.len() == 1 => {
                let payload = match vec.get(0).unwrap() {
                    Node::RawStmt(raw) => match raw.stmt.as_ref().unwrap().as_ref() {
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
                    },
                    _ => None,
                };

                (Ok(vec), payload)
            }

            // more than one Node was parsed
            Ok(vec) => (Ok(vec), None),

            // parsing failed
            Err(e) => (Err(e), None),
        };

        Some(ScannedStatement {
            sql,
            parsed,
            payload,
        })
    }

    fn scan_copy_data(&mut self) -> Option<&'a str> {
        let input = &self.sql[self.start..];

        let mut prevc = '\n' as char;
        let mut iter = input.chars().enumerate().peekable();
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
