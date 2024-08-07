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

use crate::marine_test::utils::new_ident;
use crate::marine_test::utils::itype_to_tokens;
use crate::TResult;

use marine_it_parser::it_interface::it::IType;
use marine_it_parser::it_interface::it::IFunctionArg;
use marine_it_parser::it_interface::IRecordTypes;
use marine_it_parser::it_interface::IFunctionSignature;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Copy)]
pub(super) enum CallParametersSettings {
    Default,
    UserDefined,
}

pub(super) fn generate_module_method(
    module_name: &str,
    signature: &IFunctionSignature,
    cp_setting: CallParametersSettings,
    records: &IRecordTypes,
) -> TResult<TokenStream> {
    let arguments = generate_arguments(signature.arguments.iter(), records)?;
    let output_type = generate_output_type(&signature.outputs, records)?;
    let mcall = generate_marine_call(module_name, cp_setting, signature, records)?;

    let (cp, func_name) = generate_call_parameters(&cp_setting, signature)?;

    let module_method = quote! {
        pub fn #func_name(&mut self #(,#arguments)* #cp) #output_type {
            #mcall
        }
    };

    Ok(module_method)
}

pub(super) fn generate_module_method_forward(
    signature: &IFunctionSignature,
    cp_setting: CallParametersSettings,
    records: &IRecordTypes,
) -> TResult<TokenStream> {
    let arguments = generate_arguments(signature.arguments.iter(), records)?;
    let output_type = generate_output_type(&signature.outputs, records)?;
    let mcall = generate_forward_call(cp_setting, signature)?;

    let (cp, func_name) = generate_call_parameters(&cp_setting, signature)?;

    let module_method = quote! {
        pub fn #func_name(&mut self #(,#arguments)* #cp) #output_type {
            #mcall
        }
    };

    Ok(module_method)
}

fn generate_marine_call(
    module_name: &str,
    cp_settings: CallParametersSettings,
    method_signature: &IFunctionSignature,
    records: &IRecordTypes,
) -> TResult<TokenStream> {
    let args = method_signature.arguments.iter().map(|a| a.name.as_str());
    let convert_arguments = generate_arguments_converter(args)?;

    let output_type = get_output_type(&method_signature.outputs)?;
    let set_result = generate_set_result(&output_type);
    let function_call = generate_function_call(module_name, &method_signature.name, cp_settings);
    let convert_result_to_output_type = generate_convert_to_output(&output_type, records)?;
    let ret = generate_ret(&output_type);

    let function_call = quote! {
        #convert_arguments

        #set_result #function_call

        #convert_result_to_output_type

        #ret
    };

    Ok(function_call)
}

fn generate_forward_call(
    cp_settings: CallParametersSettings,
    method_signature: &IFunctionSignature,
) -> TResult<TokenStream> {
    let mut args = method_signature
        .arguments
        .iter()
        .map(|a| new_ident(a.name.as_str()))
        .collect::<TResult<Vec<syn::Ident>>>()?;

    let method_name = if let CallParametersSettings::UserDefined = cp_settings {
        args.push(new_ident("cp")?);
        new_ident(format!("{}_cp", method_signature.name.as_str()).as_str())?
    } else {
        new_ident(method_signature.name.as_str())?
    };

    let function_call = quote! {
        self.__facade.#method_name(#(#args,)*)
    };

    Ok(function_call)
}

/// Generates type convertor to json because of AppService receives them in json.
fn generate_arguments_converter<'a>(
    args: impl ExactSizeIterator<Item = &'a str>,
) -> TResult<TokenStream> {
    let arg_idents: Vec<syn::Ident> = args.map(new_ident).collect::<Result<_, _>>()?;

    let args_converter = quote! { let arguments = marine_rs_sdk_test::internal::serde_json::json!([#(#arg_idents),*]); };

    Ok(args_converter)
}

fn generate_function_call(
    module_name: &str,
    method_name: &str,
    cp_setting: CallParametersSettings,
) -> TokenStream {
    let cp = match cp_setting {
        CallParametersSettings::Default => quote! { <_>::default() },
        CallParametersSettings::UserDefined => quote! { cp },
    };

    quote! { self.marine.as_ref().borrow_mut().call_module(#module_name, #method_name, arguments, #cp).expect("call to Marine failed"); }
}

fn generate_set_result(output_type: &Option<&IType>) -> TokenStream {
    match output_type {
        Some(_) => quote! { let result = },
        None => TokenStream::new(),
    }
}

fn generate_convert_to_output(
    output_type: &Option<&IType>,
    records: &IRecordTypes,
) -> TResult<TokenStream> {
    let result_stream = match output_type {
        Some(ty) => {
            let ty = itype_to_tokens(ty, records)?;
            quote! {
                let result: #ty = marine_rs_sdk_test::internal::serde_json::from_value(result).expect("the default deserializer shouldn't fail");
            }
        }
        None => TokenStream::new(),
    };

    Ok(result_stream)
}

fn generate_ret(output_type: &Option<&IType>) -> TokenStream {
    match output_type {
        Some(_) => quote! { result },
        None => TokenStream::new(),
    }
}

fn generate_arguments<'a, 'r>(
    arguments: impl ExactSizeIterator<Item = &'a IFunctionArg>,
    records: &'r IRecordTypes,
) -> TResult<Vec<TokenStream>> {
    arguments
        .map(|argument| -> TResult<_> {
            let arg_name = new_ident(&argument.name)?;
            let arg_type = itype_to_tokens(&argument.ty, records)?;

            let arg = quote! { #arg_name: #arg_type };
            Ok(arg)
        })
        .collect::<TResult<Vec<_>>>()
}

fn generate_output_type(output_types: &[IType], records: &IRecordTypes) -> TResult<TokenStream> {
    let output_type = get_output_type(output_types)?;
    match output_type {
        None => Ok(TokenStream::new()),
        Some(ty) => {
            let output_type = itype_to_tokens(ty, records)?;
            let output_type = quote! { -> #output_type };

            Ok(output_type)
        }
    }
}

fn get_output_type(output_types: &[IType]) -> TResult<Option<&IType>> {
    use crate::TestGeneratorError::ManyFnOutputsUnsupported;

    match output_types.len() {
        0 => Ok(None),
        1 => Ok(Some(&output_types[0])),
        _ => Err(ManyFnOutputsUnsupported),
    }
}

fn generate_call_parameters(
    cp_setting: &CallParametersSettings,
    signature: &IFunctionSignature,
) -> TResult<(TokenStream, syn::Ident)> {
    match cp_setting {
        CallParametersSettings::Default => {
            let func_name = new_ident(&signature.name)?;
            Ok((TokenStream::new(), func_name))
        }
        CallParametersSettings::UserDefined => {
            let cp = quote! { , cp: marine_rs_sdk_test::CallParameters, };
            let func_name = format!("{}_cp", signature.name);
            let func_name = new_ident(&func_name)?;
            Ok((cp, func_name))
        }
    }
}
