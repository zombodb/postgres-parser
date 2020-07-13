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

//! Contains a trait named `ConvertNode` and a few hand-written implementations
//! to allow for the conversion of Postgres' "parsenode" `Node` pointer types into
//! owned Rust types.
//!
//! Implementations for the `ConvertNode` type are (mostly) machine-generated and
//! appear in `safe.rs`
use crate::sys::NodeTag;

/// A crate-public trait for converting `crate::sys::Node`-types into
/// their `crate::nodes::*` counterparts
pub(crate) trait ConvertNode {
    /// Implementations should perform a "deep copy" to convert the "sys" node
    /// into its "safe" counterpart
    fn convert(&self) -> crate::nodes::Node;
}

impl ConvertNode for crate::sys::List {
    fn convert(&self) -> crate::nodes::Node {
        let selfptr = self as *const _ as *const crate::sys::List;
        let mut elements = Vec::new();
        for i in 0..self.length {
            let node = unsafe { crate::sys::list_nth(selfptr, i) } as *const crate::sys::Node;

            // we should never see a null element inside a list
            if !node.is_null() {
                elements.push(unsafe { node.as_ref().unwrap().convert() });
            }
        }

        crate::nodes::Node::List(elements)
    }
}

impl ConvertNode for crate::sys::Value {
    fn convert(&self) -> crate::nodes::Node {
        use crate::nodes::Value;

        fn make_string(value: &crate::sys::Value) -> String {
            let cstr = unsafe { value.val.str_.as_ref() };
            let cstr = unsafe { std::ffi::CStr::from_ptr(*cstr) };

            cstr.to_str()
                .expect("failed to convert Value::String to Rust::String")
                .to_owned()
        }

        let mut value = Value {
            string: None,
            int: None,
            float: None,
            bit_string: None,
            null: None,
        };

        match self.type_ {
            NodeTag::T_String => value.string = Some(make_string(self)),
            NodeTag::T_Integer => value.int = Some(*unsafe { self.val.ival.as_ref() }),
            NodeTag::T_Float => value.float = Some(make_string(self)),
            NodeTag::T_BitString => value.bit_string = Some(make_string(self)),
            NodeTag::T_Null => value.null = Some(()),
            _ => panic!("unexpected Value type: {:?}", self.type_),
        }

        crate::nodes::Node::Value(value)
    }
}

impl ConvertNode for crate::sys::CreateForeignTableStmt {
    fn convert(&self) -> crate::nodes::Node {
        let stmt = crate::nodes::CreateForeignTableStmt {
            base: match self.base.convert() {
                crate::Node::CreateStmt(stmt) => stmt,
                _ => panic!("could not convert sys::CreateForeignTableStmt"), // shouldn't happen
            },
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        };

        crate::nodes::Node::CreateForeignTableStmt(stmt)
    }
}
