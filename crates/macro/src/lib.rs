/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Defines the #[fce] macro that should be used with all export functions, extern blocks.
//! At now, It supports the following types that could be used as parameters in export or foreign
//! functions: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, String, Vec<u8>. Also struct
//! where all fields are public and have aforementioned types could be used as parameters. In this
//! case #[fce] should be also applied to this structs.
//!
//! # Examples
//!
//! This example shows how a function could be exported:
//! ```
//! #[fce]
//! pub fn greeting(name: String) -> String {
//!     format!("Hi {}", name)
//! }
//! ```
//!
//! This more complex example shows how a function could be imported from another Wasm module
//! and how a struct could be passed:
//!
//! ```
//! #[fce]
//! struct HostReturnValue {
//!     pub error_code: i32,
//!     pub outcome: Vec<u8>
//! }
//!
//! #[fce]
//! pub fn read_ipfs_file(file_path: String) -> HostReturnValue {
//!     let hash = calculate_hash(file_path);
//!     ipfs(hash)
//! }
//!
//! #[fce]
//! #[link(wasm_import_module = "ipfs_node.wasm")]
//! extern "C" {
//!     pub fn ipfs(file_hash: String) -> HostReturnValue;
//! }
//!
//! ```

#![doc(html_root_url = "https://docs.rs/fluence-sdk-macro/0.4.1")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "1024"]

use fluence_sdk_wit::fce as fce_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn fce(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // into converts proc_macro::TokenStream to proc_macro2::TokenStream
    match fce_impl(input.into()) {
        Ok(v) => v,
        // converts syn:error to proc_macro2::TokenStream
        Err(e) => e.to_compile_error(),
    }
    // converts proc_macro2::TokenStream to proc_macro::TokenStream
    .into()
}
