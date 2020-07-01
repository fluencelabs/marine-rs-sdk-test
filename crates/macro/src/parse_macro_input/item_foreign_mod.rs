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

use super::ParseMacroInput;
use crate::fce_ast_types;
use crate::fce_ast_types::FCEAst;

use syn::Error;
use syn::Result;
use syn::spanned::Spanned;

const LINK_DIRECTIVE_NAME: &str = "link";
const LINK_NAME_DIRECTIVE_NAME: &str = "link_name";
const WASM_IMPORT_MODULE_DIRECTIVE_NAME: &str = "wasm_import_module";

impl ParseMacroInput for syn::ItemForeignMod {
    fn parse_macro_input(self) -> Result<FCEAst> {
        match self.abi.name {
            Some(ref name) if name.value() != "C".to_string() => {
                return Err(Error::new(self.span(), "only 'C' abi is allowed"))
            }
            _ => {}
        };

        let self_span = self.span();

        let imports = self
            .items
            .into_iter()
            .map(parse_raw_foreign_item)
            .collect::<Result<_>>()?;

        // try to find and parse wasm module name from
        //   #[link(wasm_import_module = "host")]
        let wasm_import_module: Option<String> = self
            .attrs
            .into_iter()
            .filter_map(|attr| attr.parse_meta().ok())
            .filter(|meta| meta.path().is_ident(LINK_DIRECTIVE_NAME))
            .filter_map(|meta| match meta {
                syn::Meta::List(meta_list) => Some(meta_list),
                _ => None,
            })
            .filter_map(|meta_list| match meta_list.nested.first().unwrap() {
                syn::NestedMeta::Meta(meta) => Some(meta.clone()),
                _ => None,
            })
            .filter(|meta| meta.path().is_ident(WASM_IMPORT_MODULE_DIRECTIVE_NAME))
            .map(extract_value)
            .collect();

        match wasm_import_module {
            Some(namespace) => {
                let extern_mod_item = fce_ast_types::AstExternModItem { namespace, imports };
                Ok(FCEAst::ExternMod(extern_mod_item))
            }
            None => Err(Error::new(
                self_span,
                "import module name should be defined by 'wasm_import_module' directive",
            )),
        }
    }
}

fn parse_raw_foreign_item(raw_item: syn::ForeignItem) -> Result<fce_ast_types::AstFunctionItem> {
    let function_item = match raw_item {
        syn::ForeignItem::Fn(function_item) => function_item,
        _ => {
            return Err(Error::new(
                raw_item.span(),
                "#[fce] could be upplied only to a function, struct ot extern block",
            ))
        }
    };

    // parse the link_name attribute
    //  #[link_name = "put"]
    //  fn ipfs_put(ptr: i32, size: i32);
    let link_name: Option<String> = function_item
        .attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter(|meta| meta.path().is_ident(LINK_NAME_DIRECTIVE_NAME))
        .map(extract_value)
        .collect();

    let mut function_item = super::item_fn::parse_function(function_item.sig, function_item.vis)?;

    if let Some(link_name) = link_name {
        function_item.name = link_name;
    }

    Ok(function_item)
}

fn extract_value(nested_meta: syn::Meta) -> Option<String> {
    match nested_meta {
        syn::Meta::NameValue(name_value) => match name_value.lit {
            syn::Lit::Str(str) => Some(str.value()),
            _ => None,
        },
        _ => None,
    }
}