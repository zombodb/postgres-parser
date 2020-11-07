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
use bindgen::EnumVariation;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;
use syn::export::TokenStream2;
use syn::spanned::Spanned;
use syn::{Item, Type};

fn main() -> Result<(), std::io::Error> {
    if running_on_docs_rs() {
        // we're runningo n docs.rs, so just don't bother generating anything
        // our generated .rs files are under source control, so they'll already be there
        //
        // and we don't need to generate "libpostgres_parser.a" just to build docs
        return Ok(());
    }

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR unset"));

    let paths = run_build_sh(&manifest_dir)?;
    let paths: Vec<&str> = paths.rsplit(';').collect();
    let (postgres_parser_a, install_dir) = (
        PathBuf::from(paths.get(1).unwrap()),
        PathBuf::from(paths.get(0).unwrap()),
    );

    println!(
        "cargo:rustc-link-search={}",
        postgres_parser_a.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static=postgres_parser");

    bindgen(&manifest_dir, install_dir);
    Ok(())
}

fn running_on_docs_rs() -> bool {
    std::env::var("RUSTDOCFLAGS")
        .unwrap_or("".to_string())
        .contains("docs.rs")
}

fn bindgen(manifest_dir: &PathBuf, install_dir: PathBuf) {
    let mut include_h = PathBuf::from(manifest_dir);
    include_h.push("includes.h");

    let mut include_path = PathBuf::from(&install_dir);
    include_path.push("include");
    include_path.push("server");

    let mut builder = bindgen::Builder::default()
        .header(include_h.to_str().unwrap())
        .clang_arg(&format!("-I{}", include_path.display()))
        .whitelist_function("MemoryContextInit")
        // .whitelist_function("SetDatabaseEncoding")
        .whitelist_function("CopyErrorData")
        .whitelist_function("FreeErrorData")
        .whitelist_function("FlushErrorState")
        .whitelist_function("MemoryContextReset")
        .whitelist_function("AllocSetContextCreateInternal")
        .whitelist_function("raw_parser")
        .derive_debug(true)
        .derive_copy(false)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate_comments(true)
        .layout_tests(true)
        .size_t_is_usize(true)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .rustfmt_bindings(false);

    // whitelist types from "parsenodes.h"
    for typename in extract_types(&include_path).unwrap() {
        builder = builder.whitelist_type(typename);
    }

    // and a few others that don't come from there
    builder = builder
        .whitelist_type("pg_enc")
        .whitelist_type("sigjmp_buf")
        .whitelist_type("ErrorContextCallback")
        .whitelist_type("ErrorData")
        .whitelist_type("MemoryContext")
        .whitelist_type("MemoryContextData")
        .whitelist_type("Size")
        .whitelist_var("PG_exception_stack")
        .whitelist_var("error_context_stack")
        .whitelist_var("CurrentMemoryContext")
        .whitelist_var("TopMemoryContext")
        .whitelist_var("ALLOCSET_DEFAULT_.*");

    let bindings = builder
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate Rust bindings from Postgres headers"));

    let out_path = PathBuf::from(manifest_dir);

    let sys = process_bindings(bindings.to_string());
    let sys_rs = out_path.join("src").join("sys.rs");
    std::fs::write(&sys_rs, sys).unwrap_or_else(|e| panic!("Unable to save sys.rs: {:?}", e));
    rust_fmt(&sys_rs).unwrap_or_else(|_| panic!("failed to run rustfmt on rust sys bindings"));

    let nodes = generate_safe_wrappers(bindings.to_string());
    let nodes_rs = out_path.join("src").join("nodes.rs");
    std::fs::write(&nodes_rs, nodes).unwrap_or_else(|e| panic!("Unable to save safe.rs: {:?}", e));
    rust_fmt(&nodes_rs).unwrap_or_else(|_| panic!("failed to run rustfmt on rust safe bindings"));
}

fn extract_types(include_path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let parsenodes_h = include_path.join("nodes").join("parsenodes.h");
    let primnodes_h = include_path.join("nodes").join("primnodes.h");
    let mut types = Vec::new();

    for file in vec![File::open(parsenodes_h)?, File::open(primnodes_h)?] {
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("typedef") {
                    let parts: Vec<_> = line.split_whitespace().collect();
                    if let Some(typename) = parts.get(2) {
                        if "Query" == *typename
                            || "SubPlan" == *typename
                            || "RangeTblEntry" == *typename
                            || "RangeTblFunction" == *typename
                            || "Bitmapset" == *typename
                        {
                            // we don't want these types -- they're not really a parsenode, even tho
                            // it's defined there
                            continue;
                        }
                        types.push(typename.to_string());
                    }
                }
            }
        }
    }

    Ok(types)
}

fn run_build_sh(pwd: &PathBuf) -> Result<String, std::io::Error> {
    run_command(
        Command::new(&format!("{}/build.sh", pwd.to_str().unwrap())).current_dir(pwd),
        true,
    )
}

fn rust_fmt(path: &PathBuf) -> Result<(), std::io::Error> {
    run_command(
        Command::new("rustfmt")
            .arg(path.to_str().unwrap())
            .current_dir("."),
        false,
    )?;

    Ok(())
}

fn run_command(
    mut command: &mut Command,
    want_postgres_parser_a: bool,
) -> Result<String, std::io::Error> {
    eprintln!("command={:?}", command);
    command = command
        .env("NUM_CPUS", num_cpus::get().to_string())
        .env_remove("DEBUG")
        .env_remove("MAKEFLAGS")
        .env_remove("MAKELEVEL")
        .env_remove("MFLAGS")
        .env_remove("DYLD_FALLBACK_LIBRARY_PATH")
        .env_remove("OPT_LEVEL")
        .env_remove("TARGET")
        .env_remove("PROFILE")
        .env_remove("OUT_DIR")
        .env_remove("HOST")
        .env_remove("NUM_JOBS");

    let output = command.output()?;

    let mut postgres_parser_a = None;
    if !output.stdout.is_empty() {
        for line in String::from_utf8(output.stdout).unwrap().lines() {
            if line.starts_with("cargo:") {
                eprintln!("{}", line);
            } else {
                eprintln!("[stdout] {}", line);
            }

            // output postgres.a file is just the last line from stdout
            postgres_parser_a = Some(line.to_owned());
        }
    }

    if !output.stderr.is_empty() {
        for line in String::from_utf8(output.stderr).unwrap().lines() {
            eprintln!("[stderr] {}", line);
        }
    }

    if let None = postgres_parser_a {
        if want_postgres_parser_a {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not determine location of generated 'postgres.a'",
            ))
        } else {
            Ok(String::default())
        }
    } else if output.status.success() {
        Ok(postgres_parser_a.unwrap())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

fn build_node_tag_set(
    file: &syn::File,
    struct_names: HashSet<String>,
) -> HashMap<String, &syn::Variant> {
    let mut node_tags = HashMap::new();
    for item in &file.items {
        match item {
            Item::Enum(e) => {
                if "NodeTag" == e.ident.to_string() {
                    for variant in &e.variants {
                        let name = variant.ident.to_string();
                        let name = name.trim_start_matches("T_").to_string();
                        if struct_names.contains(&name) {
                            node_tags.insert(name, variant);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    node_tags
}

fn build_struct_names(file: &syn::File) -> HashSet<String> {
    let mut struct_names = HashSet::new();
    for item in &file.items {
        match item {
            Item::Struct(s) => {
                if s.ident.to_string() == "CreateForeignTableStmt"
                    || s.fields
                        .iter()
                        .find(|p| match p.ident.as_ref() {
                            Some(ident) => {
                                "type_" == ident.to_string() || "xpr" == ident.to_string()
                            }
                            None => false,
                        })
                        .is_some()
                {
                    struct_names.insert(s.ident.to_string());
                }
            }
            _ => {}
        }
    }

    struct_names
}

fn process_bindings(input: String) -> String {
    let ast = syn::parse_str::<syn::File>(&input).expect("failed to parse bindings code");
    let mut output = TokenStream2::new();

    output.extend(quote! {
        //! Generated types and constants from Postgres' header files necessary to represent
        //! a parse tree as raw "C" structures.  Also contains various enum types used by
        //! this module and the `nodes` module
        #![allow(improper_ctypes)]
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        #![allow(dead_code)]

        use serde::{Serialize, Deserialize};
    });

    generate_serde_support(&ast, &mut output);

    output.to_string()
}

fn generate_serde_support(ast: &syn::File, output: &mut TokenStream2) {
    for item in &ast.items {
        match item {
            Item::Enum(e) => output.extend(quote! {
                #[derive(Serialize, Deserialize)]
                #e
            }),
            other => output.extend(quote! {#other}),
        }
    }
}

fn generate_safe_wrappers(input: String) -> String {
    let ast = syn::parse_str::<syn::File>(&input).expect("failed to parse bindings code");
    let mut output = TokenStream2::new();

    let node_tags = build_node_tag_set(&ast, build_struct_names(&ast));
    let mut struct_names = node_tags.keys().map(|v| v.as_str()).collect::<Vec<&str>>();
    struct_names.sort();

    output.extend(quote! {
        //! Generated types to represent a parse tree in a safe manner as returned from `parse_query()`
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        #![allow(dead_code)]

        use serde::{Serialize, Deserialize};
    });

    generate_node_enum(&struct_names, &mut output);
    generate_structs(&ast, &struct_names, &mut output);
    generate_convert_trait_impls(&ast, &struct_names, &mut output);
    output.to_string()
}

fn generate_node_enum(struct_names: &Vec<&str>, output: &mut TokenStream2) {
    let mut enum_stream = TokenStream2::new();
    for name in struct_names {
        if "List" == *name {
            enum_stream.extend(quote! {
                List(Vec<Node>),
            });
        } else {
            let ident = syn::Ident::new(&name, enum_stream.span());
            enum_stream.extend(quote! {
                #ident(#ident),
            });
        }
    }

    output.extend(quote! {
        /// All the various Postgres parse tree nodes that can be returned upon parsing a SQL statement
        #[allow(non_camel_case_types)]
        #[derive(Debug, Eq, PartialEq)]
        #[derive(Serialize, Deserialize)]
        #[derive(Clone)]
        pub enum Node {
            #enum_stream
        }
    });
}

fn generate_structs(ast: &syn::File, struct_names: &Vec<&str>, output: &mut TokenStream2) {
    for item in &ast.items {
        match item {
            Item::Struct(s) => {
                let name = s.ident.to_string();
                let attributes = extract_doc_attributes(&s.attrs);

                if struct_names.contains(&name.as_str()) {
                    if "List" == name {
                        // don't need a struct for List as we just represent it as a Vec<Node>
                        continue;
                    } else if "Value" == name {
                        output.extend(quote! {
                            #[derive(Debug, Eq, PartialEq)]
                            #[derive(Serialize, Deserialize)]
                            #[derive(Clone)]
                            #attributes
                            pub struct Value {
                                #[serde(skip_serializing_if = "Option::is_none")]
                                pub string: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none")]
                                pub int: Option<i32>,
                                #[serde(skip_serializing_if = "Option::is_none")]
                                pub float: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none")]
                                pub bit_string: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none")]
                                pub null: Option<()>
                            }
                        });
                    } else {
                        generate_single_struct(s, struct_names, output);
                    }
                }
            }
            _ => {}
        }
    }
}

fn generate_single_struct(
    item: &syn::ItemStruct,
    struct_names: &Vec<&str>,
    output: &mut TokenStream2,
) {
    let struct_name = &item.ident;
    let attributes = extract_doc_attributes(&item.attrs);

    let mut fields_stream = TokenStream2::new();
    for field in &item.fields {
        let name = field.ident.as_ref().unwrap();
        let namestr = name.to_string();

        if "type_" == namestr {
            continue;
        } else if "xpr" == namestr {
            continue;
        } else if "CallStmt" == struct_name.to_string() && "funcexpr" == namestr {
            // this comes from the "CallStmt" struct
            // not sure how else to handle this case, as
            // its 'funcexpr' isn't set by the parser
            // and we don't have its corresponding 'FuncExpr' type
            continue;
        }

        match &field.ty {
            Type::Path(path) => {
                let tystr = &format!("{}", quote! {#path});
                let ty = match tystr.as_str() {
                    ":: std :: os :: raw :: c_char" => quote!(char),
                    ":: std :: os :: raw :: c_int" => quote!(i32),
                    ":: std :: os :: raw :: c_long" => quote!(i64),
                    "f64" => quote!(f64),
                    "int16" => quote!(i16),
                    "int32" => quote!(i32),
                    "uint64" => quote!(u64),
                    "bool" => quote!(bool),
                    tystr if struct_names.contains(&tystr) => quote!(#path),
                    _ => quote!(crate::sys::#path),
                };

                fields_stream.extend(quote! {
                    pub #name: #ty,
                });
            }
            Type::Ptr(ptr) => match ptr.elem.as_ref() {
                Type::Path(path) => {
                    let tystr = &format!("{}", quote! {#path});
                    if "Bitmapset" == tystr {
                        continue;
                    }
                    let ty = match tystr.as_str() {
                        ":: std :: os :: raw :: c_char" => quote!(Option<String>),
                        "List" => quote!(Option<Vec<Node>>),
                        _ => quote!(Option<Box<#path>>),
                    };

                    fields_stream.extend(quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        pub #name: #ty,
                    });
                }
                unknown => panic!("unrecognized pointer type: {:?}", unknown),
            },
            unknown => panic!("unrecognized type: {:?}", unknown),
        }
    }

    output.extend(quote! {
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[derive(Debug, Eq, PartialEq)]
        #[derive(Serialize, Deserialize)]
        #[derive(Clone)]
        #attributes
        pub struct #struct_name {
            #fields_stream
        }
    });
}

fn generate_convert_trait_impls(
    ast: &syn::File,
    struct_names: &Vec<&str>,
    output: &mut TokenStream2,
) {
    for item in &ast.items {
        match item {
            Item::Struct(s) => {
                let ident = &s.ident;
                let name = ident.to_string();

                if "Node" == name {
                    generate_convert_trait_for_node(struct_names, output);
                } else if "List" == name {
                    // we manually implemented this one
                    continue;
                } else if "Value" == name {
                    // we manually implemented this one
                    continue;
                } else if "CreateForeignTableStmt" == name {
                    // we manually implemented this one
                    continue;
                } else {
                    if struct_names.contains(&name.as_str()) {
                        let conversion = generate_convert_fn(s, struct_names);
                        output.extend(quote! {
                            impl crate::convert::ConvertNode for crate::sys::#ident {
                                fn convert(&self) -> crate::nodes::Node {
                                    #conversion
                                }
                            }
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

fn generate_convert_fn(s: &syn::ItemStruct, struct_names: &Vec<&str>) -> TokenStream2 {
    let ident = &s.ident;
    let struct_name = ident.to_string();
    let mut fields_stream = TokenStream2::new();

    for field in &s.fields {
        let name = field.ident.as_ref().unwrap();
        let namestr = name.to_string();

        if "type_" == namestr {
            continue;
        } else if "xpr" == namestr {
            continue;
        } else if "CallStmt" == struct_name && "funcexpr" == namestr {
            continue;
        }

        match &field.ty {
            Type::Path(path) => {
                let tystr = &format!("{}", quote! {#path});
                let rhs = match tystr.as_str() {
                    ":: std :: os :: raw :: c_char" => quote!(self.#name as u8 as char),
                    ":: std :: os :: raw :: c_int" => quote!(self.#name as i32),
                    ":: std :: os :: raw :: c_long" => quote!(self.#name as i64),
                    "f64" => quote!(self.#name as f64),
                    "int16" => quote!(self.#name as i16),
                    "int32" => quote!(self.#name as i32),
                    "uint64" => quote!(self.#name as u64),
                    "bool" => quote!(self.#name as bool),
                    "Value" => quote! {
                        match self.#name.convert() {
                            crate::nodes::Node::Value(value) => value,
                            _=> panic!("Value didn't convert to Value")
                        }
                    },
                    tystr if struct_names.contains(&tystr) => quote!(self.#name as #path),
                    _ => quote!(self.#name as crate::sys::#path),
                };

                fields_stream.extend(quote! {
                    #name: #rhs,
                });
            }
            Type::Ptr(ptr) => match ptr.elem.as_ref() {
                Type::Path(path) => {
                    let tystr = &format!("{}", quote! {#path});
                    if "Bitmapset" == tystr {
                        continue;
                    }

                    let conversion = match tystr.as_str() {
                        ":: std :: os :: raw :: c_char" => quote! {
                            if self.#name.is_null() {
                                None
                            } else {
                                Some(unsafe { std::ffi::CStr::from_ptr(self.#name).to_str().unwrap().to_owned() })
                            }
                        },
                        "List" => quote! {
                            if self.#name.is_null() {
                                None
                            } else {
                                match unsafe { self.#name.as_ref().unwrap().convert() } {
                                    crate::nodes::Node::List(list) => Some(list),
                                    _ => panic!("not a List!"),
                                }
                            }
                        },
                        "Node" => quote! {
                            if self.#name.is_null() {
                                None
                            } else {
                                Some(Box::new(unsafe { self.#name.as_ref().unwrap().convert() }))
                            }
                        },
                        _ => quote! {
                            if self.#name.is_null() {
                                None
                            } else {
                                match unsafe { self.#name.as_ref().unwrap().convert() } {
                                    crate::nodes::Node::#path(value) => Some(Box::new(value)),
                                    _=> panic!("{} didn't convert to {}", stringify!(#name), stringify!(#path))
                                }
                            }
                        },
                    };

                    fields_stream.extend(quote! {
                        #name: #conversion,
                    });
                }
                unknown => panic!("unrecognized pointer type: {:?}", unknown),
            },
            _ => {}
        }
    }

    let mut stream = TokenStream2::new();

    stream.extend(quote! {
        Node::#ident(#ident {
            #fields_stream
        })
    });

    stream
}

fn generate_convert_trait_for_node(struct_names: &Vec<&str>, output: &mut TokenStream2) {
    let mut match_arms = TokenStream2::new();

    for name in struct_names {
        let tag = syn::Ident::new(&format!("T_{}", name), proc_macro2::Span::call_site());
        let ident = syn::Ident::new(*name, proc_macro2::Span::call_site());
        match_arms.extend(quote! {
            crate::sys::NodeTag::#tag => {
                let ptr = self as *const _ as *const crate::sys::#ident;
                unsafe { ptr.as_ref().unwrap().convert() }
            },
        });
    }

    output.extend(quote! {
        impl crate::convert::ConvertNode for crate::sys::Node {
            fn convert(&self) -> crate::nodes::Node {
                match self.type_ {
                    #match_arms
                    crate::sys::NodeTag::T_String |
                    crate::sys::NodeTag::T_Integer |
                    crate::sys::NodeTag::T_Float |
                    crate::sys::NodeTag::T_BitString |
                    crate::sys::NodeTag::T_Null => {
                        let ptr = self as *const _ as *const crate::sys::Value;
                        unsafe { ptr.as_ref().unwrap().convert() }
                    },
                    _ => panic!("Unrecognized NodeTag: {:?}", self.type_)
                }
            }
        }
    });
}

fn extract_doc_attributes(attributes: &Vec<syn::Attribute>) -> TokenStream2 {
    let mut stream = TokenStream2::new();

    for att in attributes {
        let str = format!("{}", quote!(#att));

        if str.starts_with("# [ doc =") {
            stream.extend(quote!(#att))
        }
    }

    stream
}
