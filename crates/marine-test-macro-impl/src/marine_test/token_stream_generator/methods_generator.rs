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

use super::methods_generator_utils::*;
use crate::TResult;

use marine_it_parser::it_interface::IFunctionSignature;
use marine_it_parser::it_interface::IRecordTypes;

use itertools::Itertools;

pub(super) fn generate_module_methods<'m, 'r>(
    module_name: &str,
    method_signatures: impl ExactSizeIterator<Item = &'m IFunctionSignature>,
    records: &'r IRecordTypes,
) -> TResult<Vec<proc_macro2::TokenStream>> {
    use CallParametersSettings::*;

    let methods_count = 2 * method_signatures.len();
    method_signatures
        .sorted_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
        .try_fold::<_, _, TResult<_>>(
            Vec::with_capacity(methods_count),
            |mut methods, signature| {
                let default_cp = generate_module_method(module_name, signature, Default, records)?;
                let user_cp = generate_module_method(module_name, signature, UserDefined, records)?;

                methods.push(default_cp);
                methods.push(user_cp);

                Ok(methods)
            },
        )
}

pub fn generate_facade_methods<'m, 'r>(
    method_signatures: impl ExactSizeIterator<Item = &'m IFunctionSignature>,
    records: &'r IRecordTypes,
) -> TResult<Vec<proc_macro2::TokenStream>> {
    use CallParametersSettings::*;

    let methods_count = 2 * method_signatures.len();
    method_signatures
        .sorted_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
        .try_fold::<_, _, TResult<_>>(
            Vec::with_capacity(methods_count),
            |mut methods, signature| {
                let default_cp = generate_module_method_forward(signature, Default, records)?;
                let user_cp = generate_module_method_forward(signature, UserDefined, records)?;

                methods.push(default_cp);
                methods.push(user_cp);

                Ok(methods)
            },
        )
}
