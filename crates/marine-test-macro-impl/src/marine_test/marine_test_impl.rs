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
