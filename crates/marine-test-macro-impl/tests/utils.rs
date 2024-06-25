/*
 * Fluence Marine Rust test SDK
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

use marine_test_macro_impl::marine_test_impl;

use marine_macro_testing_utils::{items_from_file, stream_from_file, to_syn_item};

use std::cmp::min;
use std::cmp::max;
use std::path::Path;

pub fn test_marine_test_token_streams<FP, EP>(
    marine_path: FP,
    expanded_path: EP,
    config_path: &str,
) -> bool
where
    FP: AsRef<Path>,
    EP: AsRef<Path>,
{
    let marine_item = stream_from_file(&marine_path);
    let test_token_stream = quote::quote! { #marine_item };
    let buf = marine_path.as_ref().to_path_buf();
    let attrs = quote::quote! {
        config_path = #config_path,
    };
    let marine_token_streams = marine_test_impl(
        attrs,
        test_token_stream,
        buf.parent().unwrap().to_path_buf(),
    )
    .unwrap_or_else(|e| panic!("failed to apply the marine macro due {}", e));

    let expanded_item = items_from_file(&expanded_path);
    let marine_item = to_syn_item(marine_token_streams.clone());

    if expanded_item != marine_item {
        print_token_streams_with_diff(&marine_token_streams, &expanded_path);
    }

    marine_item == expanded_item
}

pub struct TestServiceDescription {
    pub config_path: &'static str,
    pub name: &'static str,
}

pub fn test_marine_test_token_streams_multiservice<FP, EP>(
    marine_path: FP,
    expanded_path: EP,
    services: Vec<TestServiceDescription>,
) -> bool
where
    FP: AsRef<Path>,
    EP: AsRef<Path>,
{
    let marine_item = stream_from_file(&marine_path);
    let test_token_stream = quote::quote! { #marine_item };
    let buf = marine_path.as_ref().to_path_buf();
    let service_declarations = services
        .iter()
        .map(|desc| {
            let config_path = desc.config_path;
            let name = syn::parse_str::<syn::Ident>(desc.name)?;
            Ok(quote::quote! {#name(config_path = #config_path)})
        })
        .collect::<Result<Vec<_>, syn::Error>>()
        .unwrap_or_else(|e| panic!("failed to parse test arguments due to {}", e));

    let attrs = quote::quote! {
            #(#service_declarations,)*
    };

    let marine_token_streams = marine_test_impl(
        attrs,
        test_token_stream,
        buf.parent().unwrap().to_path_buf(),
    )
    .unwrap_or_else(|e| panic!("failed to apply the marine macro due {}", e));

    let expanded_item = items_from_file(&expanded_path);
    let marine_item = to_syn_item(marine_token_streams.clone());

    if expanded_item != marine_item {
        print_token_streams_with_diff(&marine_token_streams, &expanded_path);
    }

    marine_item == expanded_item
}

fn print_token_streams_with_diff<P: AsRef<Path>>(
    macro_output: &proc_macro2::TokenStream,
    expanded_path: P,
) {
    let actual = macro_output.to_string();
    let expected = stream_from_file(&expanded_path).to_string();
    let min_len = min(actual.len(), expected.len());
    let max_len = max(actual.len(), expected.len());
    let mut first_diff_index: usize = min_len;
    for i in 0..min_len {
        // String does not implement index access, but implements range access
        if actual[i..i + 1] != expected[i..i + 1] {
            first_diff_index = i;
            break;
        }
    }
    let diff = " ".repeat(first_diff_index) + &"^".repeat(max_len - first_diff_index);

    println!("actual  : {}", &actual);
    println!("expected: {}", &expected);
    println!("diff    : {}", &diff);
}
