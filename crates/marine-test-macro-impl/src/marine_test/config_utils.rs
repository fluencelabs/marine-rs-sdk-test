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
