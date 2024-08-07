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

use crate::attributes::ServiceDescription;
use crate::TResult;
use crate::TestGeneratorError;
use crate::marine_test::config_utils::Module;
use crate::marine_test::config_utils::load_config;
use crate::marine_test::modules_linker;
use crate::marine_test::config_utils;
use crate::marine_test::modules_linker::LinkedModules;
use super::service_generation_utils::generate_service_definition;

use fluence_app_service::AppServiceConfig;
use marine_it_parser::it_interface::IModuleInterface;
use proc_macro2::TokenStream;
use itertools::Itertools;

use std::path::Path;
use std::iter::zip;

pub(crate) fn generate_service_definitions(
    services: impl IntoIterator<Item = (String, ServiceDescription)>,
    file_path: &Path,
    file_path_for_app_service: &Path,
) -> TResult<Vec<TokenStream>> {
    let services = services
        .into_iter()
        .sorted_by(|lhs, rhs| lhs.0.cmp(&rhs.0)) // deterministic output required for tests
        .map(|(name, service)| ProcessedService::new(service, name, file_path))
        .collect::<TResult<Vec<ProcessedService>>>()?;

    let service_modules = services
        .iter()
        .map(|service| {
            let modules = config_utils::collect_modules(&service.config)?;
            Ok(modules)
        })
        .collect::<TResult<Vec<Vec<Module<'_>>>>>()?;

    let link_info = link_services(zip(&services, &service_modules))?;
    services
        .iter()
        .map(|service| -> TResult<TokenStream> {
            // entry with service.name was added in link_services(...), so unwrap is safe
            generate_service_definition(
                service,
                link_info.get::<str>(&service.name).unwrap(),
                file_path_for_app_service,
            )
        })
        .collect::<TResult<Vec<TokenStream>>>()
}

pub(super) fn get_facace<'modules, 'm>(
    modules: &'modules [Module<'m>],
) -> TResult<&'modules Module<'m>> {
    match modules.last() {
        Some(module) => Ok(module),
        None => Err(TestGeneratorError::NoModulesInService),
    }
}

pub(super) struct ProcessedService {
    pub(super) config: AppServiceConfig,
    pub(super) config_path: String,
    pub(super) name: String,
}

impl ProcessedService {
    pub(crate) fn new(
        service: ServiceDescription,
        name: String,
        file_path: &Path,
    ) -> TResult<Self> {
        crate::marine_test::utils::maybe_warn_about_modules_dir(&service);
        let config_wrapper = load_config(&service.config_path, file_path)?;

        Ok(Self {
            config: config_wrapper,
            config_path: service.config_path,
            name,
        })
    }
}

fn link_services<'modules>(
    services: impl ExactSizeIterator<
        Item = (&'modules ProcessedService, &'modules Vec<Module<'modules>>),
    >,
) -> TResult<LinkedModules<'modules>> {
    let facade_modules = services
        .map(|(service, modules)| {
            let facade = get_facace(modules)?;
            Ok((service.name.as_str(), &facade.interface))
        })
        .collect::<TResult<Vec<(&str, &IModuleInterface)>>>()?;

    modules_linker::link_modules(facade_modules.iter().copied())
}
