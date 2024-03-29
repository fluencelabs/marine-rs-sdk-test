/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::attributes::{MTestAttributes, ServiceDescription};
use crate::TResult;
use crate::marine_test::glue_code_generator::generate_test_glue_code;
use crate::marine_test::glue_code_generator::generate_marine_test_env_for_build_script;

use proc_macro2::TokenStream;
use darling::FromMeta;
use darling::ast::NestedMeta;
use syn::parse::Parser;

use std::path::{PathBuf, Path};

pub fn marine_test_impl(
    attrs: TokenStream,
    input: TokenStream,
    file_path: PathBuf,
) -> TResult<TokenStream> {
    // from https://github.com/dtolnay/syn/issues/788
    let parser = syn::punctuated::Punctuated::<NestedMeta, syn::Token![,]>::parse_terminated;
    let attrs = parser.parse2(attrs)?;
    let attrs: Vec<NestedMeta> = attrs.into_iter().collect();
    let attrs = MTestAttributes::from_list(&attrs)?;

    let item = syn::parse2::<syn::Item>(input)?;

    generate_test_glue_code(item, attrs, file_path)
}

pub fn generate_marine_test_env_impl(
    services: impl IntoIterator<Item = (String, ServiceDescription)>,
    build_rs_file_path: &Path,
) -> TResult<TokenStream> {
    generate_marine_test_env_for_build_script(services, build_rs_file_path)
}
