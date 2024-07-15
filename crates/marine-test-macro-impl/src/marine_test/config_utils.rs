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

use crate::{TResult, TestGeneratorError};

use fluence_app_service::AppServiceConfig;
use fluence_app_service::TomlAppServiceConfig;
use marine_it_parser::module_it_interface;
use marine_it_parser::it_interface::IModuleInterface;

use std::convert::TryInto;
use std::path::{PathBuf, Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Module<'m> {
    pub name: &'m str,
    pub interface: IModuleInterface,
}

impl<'m> Module<'m> {
    fn new(name: &'m str, interface: IModuleInterface) -> Self {
        Self { name, interface }
    }
}

pub(crate) fn load_config(config_path: &str, file_path: &Path) -> TResult<AppServiceConfig> {
    let config_path_buf = file_path.join(config_path);

    let mut marine_config = TomlAppServiceConfig::load(&config_path_buf)
        .map_err(|e| TestGeneratorError::ConfigLoadError(config_path_buf.clone(), e))?;

    marine_config.toml_marine_config.base_path = config_path_buf
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default();

    let marine_config = marine_config
        .try_into()
        .map_err(|e| TestGeneratorError::ConfigLoadError(config_path_buf, e))?;

    Ok(marine_config)
}

/// Returns all modules the provided config consists of.
pub(super) fn collect_modules(config: &AppServiceConfig) -> TResult<Vec<Module<'_>>> {
    let module_paths = collect_module_paths(config)?;

    module_paths
        .into_iter()
        .map(|(name, path)| {
            module_it_interface(&path)
                .map(|interface| Module::new(name, interface))
                .map_err(|e| TestGeneratorError::ITParserError(path.to_owned(), e))
        })
        .collect::<TResult<Vec<_>>>()
}

fn collect_module_paths(config: &AppServiceConfig) -> TResult<Vec<(&str, PathBuf)>> {
    config
        .marine_config
        .modules_config
        .iter()
        .map(|m| {
            m.get_path(&config.marine_config.modules_dir)
                .map(|path| (m.import_name.as_str(), path))
                .map_err(|e| TestGeneratorError::ModuleResolveError(m.import_name.to_owned(), e))
        })
        .collect::<TResult<Vec<_>>>()
}
