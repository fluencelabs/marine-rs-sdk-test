/*
 * Marine Rust test SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![doc(html_root_url = "https://docs.rs/marine-test-macro/0.8.1")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![feature(proc_macro_span)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "1024"]

use marine_test_macro_impl::marine_test_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::spanned::Spanned;

/// This macro allows user to write tests for services in the following form:
///```rust
/// #[marine_test(config = "/path/to/Config.toml", modules_dir = "path/to/service/modules")]
/// fn test(greeting: marine_test_env::greeting::ModuleInterface) {
///     let service_result = greeting.greeting("John".to_string());
///     assert_eq!(&service_result, "Hi, name!");
/// }
///```
#[proc_macro_error]
#[proc_macro_attribute]
pub fn marine_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let attrs_span = attrs.span();
    // here it obtains a path to the current file where macro is applied
    let mut file_path = proc_macro::Span::call_site().source_file().path();
    let _ = file_path.pop();

    match marine_test_impl(attrs, input.into(), file_path) {
        Ok(stream) => stream.into(),
        Err(e) => proc_macro_error::abort!(attrs_span, format!("{}", e)),
    }
}

// deprecated macro for backwards compatibility
#[deprecated(since = "0.6.2", note = "please use the #[marine] macro instead")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn fce_test(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = attrs.into();
    let attrs_span = attrs.span();
    // here it obtains a path to the current file where macro is applied
    let mut file_path = proc_macro::Span::call_site().source_file().path();
    let _ = file_path.pop();

    match marine_test_impl(attrs, input.into(), file_path) {
        Ok(stream) => stream.into(),
        Err(e) => proc_macro_error::abort!(attrs_span, format!("{}", e)),
    }
}
