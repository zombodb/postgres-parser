use crate::traits::ConvertNode;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref MEMORY_CONTEXT: () = {
        extern "C" {
            fn MemoryContextInit();
            fn SetDatabaseEncoding(enc: i32);
        }

        unsafe {
            SetDatabaseEncoding(crate::sys::pg_enc::PG_UTF8 as i32);
            MemoryContextInit();
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PgParserError {
    InternalNull,
    NotAList,
    ParseError { message: String, cursor_pos: i32 },
}

pub fn parse_query(statements: &str) -> std::result::Result<Vec<crate::safe::Node>, PgParserError> {
    type SigjmpBuf = [::std::os::raw::c_int; 38usize];

    #[cfg(target_os = "linux")]
    extern "C" {
        #[link_name = "__sigsetjmp"]
        pub fn sigsetjmp(env: *mut SigjmpBuf, savemask: std::os::raw::c_int)
            -> std::os::raw::c_int;
    }

    #[cfg(target_os = "macos")]
    extern "C" {
        pub fn sigsetjmp(env: *mut SigjmpBuf, savemask: std::os::raw::c_int)
            -> std::os::raw::c_int;
    }

    extern "C" {
        ///
        /// CopyErrorData --- obtain a copy of the topmost error stack entry
        ///
        /// This is only for use in error handler code.  The data is copied into the
        /// current memory context, so callers should always switch away from
        /// ErrorContext first; otherwise it will be lost when FlushErrorState is done.
        ///
        fn CopyErrorData() -> *mut crate::sys::ErrorData;

        ///
        /// FreeErrorData --- free the structure returned by CopyErrorData.
        ///
        /// Error handlers should use this in preference to assuming they know all
        /// the separately-allocated fields.
        ///
        fn FreeErrorData(data: *mut crate::sys::ErrorData);

        ///
        /// FlushErrorState --- flush the error state after error recovery
        ///
        /// This should be called by an error handler after it's done processing
        /// the error; or as soon as it's done CopyErrorData, if it intends to
        /// do stuff that is likely to provoke another error.  You are not "out" of
        /// the error subsystem until you have done this.
        ///
        fn FlushErrorState();

        ///
        /// raw_parser
        ///		Given a query in string form, do lexical and grammatical analysis.
        ///
        /// Returns a list of raw (un-analyzed) parse trees.  The immediate elements
        /// of the list are always RawStmt nodes.
        ///
        fn raw_parser(str: *const std::os::raw::c_char) -> *mut crate::sys::List;
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
            Ok(raw_parser(str))
        } else {
            // Postgres raised an ERROR and we handle it here

            // first off, restore Postgres' understanding of where it really wanted to jump to
            crate::sys::PG_exception_stack = prev_exception_stack;
            crate::sys::error_context_stack = prev_error_context_stack;

            // and now we'll make a copy of the current "ErrorData"
            let error_data_ptr = CopyErrorData();
            let error_data = error_data_ptr.as_ref().expect("could not CopyErrorData");

            // so we can pull out the details of the error
            let message = std::ffi::CStr::from_ptr(error_data.message);
            let cursor_pos = error_data.cursorpos;

            // and convert it into a PgParserError::ParseError
            let result = Err(PgParserError::ParseError {
                message: message
                    .to_str()
                    .expect("failed to convert parse error message into a &str")
                    .to_string(),
                cursor_pos,
            });

            // make sure to cleanup after ourselves
            FreeErrorData(error_data_ptr);

            FlushErrorState();

            // and return the error
            result
        }
    }

    // make sure Postgres' MemoryContext system is initialized
    let _ = *MEMORY_CONTEXT;

    match std::ffi::CString::new(statements) {
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
                        // and that worked, so we return its contained Vec<Node>
                        crate::safe::Node::List(vec) => Ok(vec),

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
    }
}
