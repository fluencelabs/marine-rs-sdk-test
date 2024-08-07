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

use crate::TResult;
use crate::ServiceDescription;

use marine_it_parser::it_interface::IRecordTypes;
use marine_it_parser::it_interface::it::IType;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn new_ident(ident_str: &str) -> TResult<syn::Ident> {
    let ident_str = ident_str.replace('-', "_");
    syn::parse_str::<syn::Ident>(&ident_str).map_err(Into::into)
}

pub(super) fn itype_to_tokens(itype: &IType, records: &IRecordTypes) -> TResult<TokenStream> {
    let token_stream = match itype {
        IType::Record(record_id) => {
            let record = records
                .get(record_id)
                .ok_or(crate::errors::CorruptedITSection::AbsentRecord(*record_id))?;
            let record_name = new_ident(&record.name)?;
            let token_stream = quote! { #record_name };
            token_stream
        }
        IType::Array(ty) => {
            let inner_ty_token_stream = itype_to_tokens(ty, records)?;
            let token_stream = quote! { Vec<#inner_ty_token_stream> };
            token_stream
        }
        IType::String => quote! { String },
        IType::ByteArray => quote! { Vec<u8> },
        IType::Boolean => quote! { bool },
        IType::S8 => quote! { i8 },
        IType::S16 => quote! { i16 },
        IType::S32 => quote! { i32 },
        IType::S64 => quote! { i64 },
        IType::U8 => quote! { u8 },
        IType::U16 => quote! { u16 },
        IType::U32 => quote! { u32 },
        IType::U64 => quote! { u64 },
        IType::I32 => quote! { i32 },
        IType::I64 => quote! { i64 },
        IType::F32 => quote! { f32 },
        IType::F64 => quote! { f64 },
    };

    Ok(token_stream)
}

pub(crate) fn maybe_warn_about_modules_dir(service: &ServiceDescription) {
    if service.modules_dir.is_some() {
        println!(
            r#"WARNING: #[marine-test] macro attribute "modules_dir" is deprecated. It will not be used by macro. Please specify loading options in config file."#,
        )
    }
}
