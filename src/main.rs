extern "C" {
    fn raw_parser(statement: *mut std::os::raw::c_char) -> *mut std::ffi::c_void;
    fn MemoryContextInit();
    fn nodeToString(node: *mut std::ffi::c_void) -> *mut std::os::raw::c_char;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    unsafe {
        MemoryContextInit();
        let query = std::ffi::CString::new(args.get(1).expect("no query specified").as_str()).unwrap();
        let stmt_list = raw_parser(query.into_raw());

        let output = nodeToString(stmt_list);
        let cstr = std::ffi::CStr::from_ptr(output);
        println!("{}", cstr.to_str().expect("failed to convert nodeToString() output to Rust string"));
    }
}