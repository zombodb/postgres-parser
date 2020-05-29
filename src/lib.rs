#[allow(improper_ctypes)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod sys;

mod safe;
mod traits;

use lazy_static::lazy_static;
use std::fmt::Debug;

pub use safe::*;
pub use traits::*;

lazy_static! {
    static ref MEMORY_CONTEXT: () = {
        extern "C" {
            pub fn MemoryContextInit();
        }

        unsafe { MemoryContextInit() }
    };
}

#[derive(Debug)]
pub enum PgParserError {
    InternalNull,
    NotAList,
    ParseError,
}

pub fn parse_query(statements: &str) -> std::result::Result<Vec<crate::safe::Node>, PgParserError> {
    extern "C" {
        pub fn raw_parser(str: *const std::os::raw::c_char) -> *mut crate::sys::List;
    }

    // make sure Postgres' MemoryContext system is initialized
    let _ = *MEMORY_CONTEXT;

    match std::ffi::CString::new(statements) {
        Ok(c_str) => {
            let parse_list = unsafe { raw_parser(c_str.as_ptr()) };
            if parse_list.is_null() {
                Ok(Vec::new())
            } else {
                match unsafe { parse_list.as_ref().unwrap().convert() } {
                    crate::safe::Node::List(vec) => Ok(vec),
                    _ => Err(PgParserError::NotAList),
                }
            }
        }
        Err(_) => Err(PgParserError::InternalNull),
    }
}
