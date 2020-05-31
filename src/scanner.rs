#[derive(Debug)]
pub struct ScannedStatement<'a> {
    pub sql: &'a str,
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

        let mut in_sl_comment = false;
        let mut in_ml_comment = false;
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut current_dollar_quote = None;

        let input = &self.sql[self.start..];
        let mut iter = input.chars().enumerate().peekable();
        loop {
            let (mut idx, c) = match iter.next() {
                Some((idx, c)) => (idx, c),
                None => break,
            };

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
                                    } else if c.is_whitespace() {
                                        // dollar quotes can't contain spaces
                                        incomplete = true;
                                        break;
                                    }
                                }
                                None => {
                                    incomplete = true;
                                    break;
                                }
                            }
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

                        let statement = ScannedStatement {
                            sql: &input[..=idx],
                        };

                        // this is where the next statement will start, if there is one
                        self.start += idx + 1;

                        return Some(statement);
                    }
                }

                // otherwise we don't care about the character
                _ => {}
            }
        }

        if self.start < self.sql.trim_end().len() {
            // we have a trailing statement that didn't end with a semicolon
            let result = Some(ScannedStatement { sql: input });

            // and we're done here
            self.start += input.len();

            result
        } else {
            None
        }
    }
}
