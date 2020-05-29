use crate::sys::NodeTag;

pub trait ConvertNode {
    fn convert(&self) -> crate::safe::Node;
}

impl ConvertNode for crate::sys::List {
    fn convert(&self) -> crate::safe::Node {
        extern "C" {
            fn list_nth(list: *const crate::sys::List, n: i32) -> *mut std::os::raw::c_void;
        }

        let selfptr = self as *const _ as *const crate::sys::List;
        let mut elements = Vec::new();
        for i in 0..self.length {
            let node = unsafe { list_nth(selfptr, i) } as *const crate::sys::Node;

            // we should never see a null element inside a list
            if !node.is_null() {
                elements.push(unsafe { node.as_ref().unwrap().convert() });
            }
        }

        crate::safe::Node::List(elements)
    }
}

impl ConvertNode for crate::sys::Value {
    fn convert(&self) -> crate::safe::Node {
        use crate::safe::Value;

        fn make_string(value: &crate::sys::Value) -> String {
            let cstr = unsafe { value.val.str.as_ref() };
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

        crate::safe::Node::Value(value)
    }
}
