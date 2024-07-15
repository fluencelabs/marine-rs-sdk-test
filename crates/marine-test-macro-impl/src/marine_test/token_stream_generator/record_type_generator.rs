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

use crate::marine_test::utils;
use crate::TResult;

use marine_it_parser::it_interface::it::IRecordFieldType;
use marine_it_parser::it_interface::IRecordTypes;

use proc_macro2::TokenStream;
use quote::quote;

use crate::marine_test::modules_linker::{LinkedModule, RecordEntry, UseDescription};
use itertools::Itertools;

pub(super) fn generate_records(
    linked_module: &LinkedModule<'_>,
    import_generator: fn(info: &UseDescription<'_>) -> TResult<TokenStream>,
) -> TResult<Vec<TokenStream>> {
    linked_module.records
        .iter()
        .sorted()
        .map(|record| -> TResult<_> {
            use RecordEntry::*;
            match record {
                Use(use_info) => import_generator(use_info),
                Declare(record) => {
                    let record_name_ident = utils::new_ident(&record.record_type.name)?;
                    let fields = prepare_field(record.record_type.fields.iter(), record.records)?;

                    Ok(quote! {
                        #[derive(Clone, Debug, marine_rs_sdk_test::internal::serde::Serialize, marine_rs_sdk_test::internal::serde::Deserialize,)]
                        #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                        pub struct #record_name_ident {
                            #(pub #fields,)*
                        }
                    })
                }
            }
        })
        .collect::<TResult<Vec<_>>>()
}

fn prepare_field<'f>(
    fields: impl ExactSizeIterator<Item = &'f IRecordFieldType>,
    records: &IRecordTypes,
) -> TResult<Vec<TokenStream>> {
    fields
        .map(|field| -> TResult<_> {
            let field_name = utils::new_ident(&field.name)?;
            let field_type = utils::itype_to_tokens(&field.ty, records)?;

            let generated_field = quote! { #field_name: #field_type };

            Ok(generated_field)
        })
        .collect::<TResult<Vec<_>>>()
}
