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
//! Provides a safe function (`parse_query()`) that parses SQL statements.
use crate::convert::ConvertNode;
use crate::PgParserError;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PARSER_LOCK: Mutex<()> = Mutex::new(());
    static ref ONETIME_SETUP: () = {
        unsafe {
            crate::sys::MemoryContextInit();
            crate::sys::SetDatabaseEncoding(crate::sys::pg_enc::PG_UTF8 as i32);
        }
    };
}

/// Parse a string of delimited SQL statements.
///
/// This function should be used to parse single statements, or even multiple statements, that
/// are (hopefully) known to already be syntatically correct.
///
/// `parse_query()` passes the entire string of `statements` to Postgres' `raw_parser()`, returning
/// an `Ok(Vec<postgres-parser::Node>)` types.
///
/// If one of the statements fails to parse, then it is considered that all the statements failed
/// to parse.  As such, a single `Err(PgParserError)` will be returned.
///
/// To parse multiple statements and evaluate their parsing success individually, you likely want
/// to use `postgres-parser::SqlStatementScanner` instead.
///
/// ## Examples
///
/// An example of parsing a single query:
/// ```rust
/// use postgres_parser::*;
/// let parse_list = parse_query("SELECT 1");
///
/// match parse_list {
///     // dump the contents of the Vec of nodes, which will only have one
///     Ok(vec) => println!("{:?}", vec),
///     Err(e) => panic!(e)
/// }
/// ```
///
/// Parsing multiple queries is exactly the same:
/// ```rust
/// use postgres_parser::*;
/// let parse_list = parse_query("SELECT 1; SELECT 2; SELECT 3");
///
/// match parse_list {
///     // dump the contents of the Vec of nodes, which will have three elements
///     Ok(vec) => println!("{:?}", vec),
///     Err(e) => panic!(e)
/// }
/// ```
///
/// But if one of those is isn't syntatically correct:
/// ```rust
/// use postgres_parser::*;
/// let parse_list = parse_query("SELECT 1; invalid query; SELECT 3");
///
/// match parse_list {
///     Ok(vec) => println!("{:?}", vec),
///
///     // we received an Err because one of the queries didn't parse
///     Err(e) => assert!(true)
/// }
/// ```
///
/// Parsing nothing (or just a `;`) returns an empty Vec:
/// ```rust
/// use postgres_parser::*;
/// let parse_list = parse_query(";").unwrap();
/// assert!(parse_list.is_empty())
/// ```
pub fn parse_query(statements: &str) -> std::result::Result<Vec<crate::Node>, PgParserError> {
    #[cfg(target_os = "linux")]
    extern "C" {
        #[link_name = "__sigsetjmp"]
        pub fn sigsetjmp(
            env: *mut crate::sys::sigjmp_buf,
            savemask: std::os::raw::c_int,
        ) -> std::os::raw::c_int;
    }

    #[cfg(target_os = "macos")]
    extern "C" {
        pub fn sigsetjmp(
            env: *mut crate::sys::sigjmp_buf,
            savemask: std::os::raw::c_int,
        ) -> std::os::raw::c_int;
    }

    //
    // a wrapper around Postgres' "raw_parser()" function that sets up a jump point
    // so we can translate possible Postgres elog(ERROR)s during parsing into proper
    // Rust Result:Err
    //
    unsafe fn raw_parser_wrapper(
        str: *const std::os::raw::c_char,
    ) -> Result<*mut crate::sys::List, PgParserError> {
        // remember Postgres' error stack
        let prev_exception_stack = crate::sys::PG_exception_stack;
        let prev_error_context_stack = crate::sys::error_context_stack;

        // because we're going to set a jump point here that'll override where
        // Postgres thinks it should jump in the event of an ERROR
        let mut jmp_buff = std::mem::MaybeUninit::uninit();
        let jump_value = sigsetjmp(jmp_buff.as_mut_ptr(), 0);

        if jump_value == 0 {
            // tell Postgres that it should jump back to us if it has an error
            crate::sys::PG_exception_stack = jmp_buff.as_mut_ptr();

            // parse the query and return a successful response if it doesn't raise an ERROR
            Ok(crate::sys::raw_parser(str))
        } else {
            // Postgres raised an ERROR and we handle it here

            // first off, restore Postgres' understanding of where it really wanted to jump to
            crate::sys::PG_exception_stack = prev_exception_stack;
            crate::sys::error_context_stack = prev_error_context_stack;

            // and now we'll make a copy of the current "ErrorData"
            let error_data_ptr = crate::sys::CopyErrorData();
            let error_data = error_data_ptr
                .as_ref()
                .expect("CopyErrorData returned null"); // error_data_ptr should never be null

            let result = if error_data.message.is_null() {
                // we have no error message
                PgParserError::UnknownParseError
            } else {
                // pull out the details of the error
                let message = std::ffi::CStr::from_ptr(error_data.message);
                let cursor_pos = error_data.cursorpos;

                // and convert it into a PgParserError::ParseError
                PgParserError::ParseError {
                    message: message
                        .to_str()
                        .expect("failed to convert parse error message into a &str")
                        .to_string(),
                    cursor_pos,
                }
            };

            // make sure to cleanup after ourselves
            crate::sys::FreeErrorData(error_data_ptr);
            crate::sys::FlushErrorState();

            // and return the error
            Err(result)
        }
    }

    // all access to the parser must be synchronized
    let _mutex = PARSER_LOCK.lock();

    // make sure Postgres' MemoryContext system is initialized
    let _ = *ONETIME_SETUP;

    // create and switch to a new memory context so that we can free it without
    // damaging anything that might be allocated by Postgres in Postgres' TopMemoryContext,
    // which is what CurrentMemoryContext should be pointing to
    let (memory_context, old_context) = unsafe {
        assert_eq!(
            crate::sys::CurrentMemoryContext,
            crate::sys::TopMemoryContext
        );

        let our_context = crate::sys::AllocSetContextCreateInternal(
            crate::sys::TopMemoryContext,
            std::ffi::CStr::from_bytes_with_nul(b"parser context\0")
                .unwrap()
                .as_ptr(),
            crate::sys::ALLOCSET_DEFAULT_MINSIZE as crate::sys::Size,
            crate::sys::ALLOCSET_DEFAULT_INITSIZE as crate::sys::Size,
            crate::sys::ALLOCSET_DEFAULT_MAXSIZE as crate::sys::Size,
        );

        let old_context = crate::sys::CurrentMemoryContext;
        crate::sys::CurrentMemoryContext = our_context;

        (our_context, old_context)
    };

    let result = match std::ffi::CString::new(statements) {
        // we have a valid query &str we can represent as a CString, so lets parse it
        Ok(c_str) => match unsafe { raw_parser_wrapper(c_str.as_ptr()) } {
            // it successfully parsed...
            Ok(parse_list) => {
                if parse_list.is_null() {
                    // but we didn't get a query back.  user probably tried to parse: ";"
                    Ok(Vec::new())
                } else {
                    // we did get a query, so lets convert it into a Node::List
                    match unsafe { parse_list.as_ref().unwrap().convert() } {
                        // and that worked, so build up a new Vec of Nodes from the
                        // contained RawStmts
                        crate::nodes::Node::List(vec) => {
                            let mut raw_statements = Vec::new();
                            let mut err = false;
                            for node in vec {
                                match node {
                                    crate::Node::RawStmt(mut rawstmt) => {
                                        raw_statements.push(*rawstmt.stmt.take().unwrap())
                                    }
                                    _ => err = true,
                                }
                            }

                            if err {
                                Err(PgParserError::NotARawStmt)
                            } else {
                                Ok(raw_statements)
                            }
                        }

                        // it didn't convert into a Node::List.  This seems pretty impossible
                        // but need to handle it anyways
                        _ => Err(PgParserError::NotAList),
                    }
                }
            }

            // it didn't successfully parse, so just return that error
            Err(e) => Err(e),
        },

        // we don't have a valid query &str we can represent as a CString
        Err(_) => Err(PgParserError::InternalNull),
    };

    // we've copied the result of the parser into owned Rust memory, so
    // free up whatever Postgres (the parser) might have allocated in our
    // MemoryContext and switch back to the previous one
    unsafe {
        crate::sys::MemoryContextReset(memory_context);
        crate::sys::CurrentMemoryContext = old_context;
    }

    result
}
