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

use darling::FromMeta;
use darling::ast::NestedMeta;

use std::collections::HashMap;

/// Describes attributes of `marine_test` macro.
#[derive(Debug, Clone)]
pub(crate) enum MTestAttributes {
    SingleService(ServiceDescription),
    MultipleServices(HashMap<String, ServiceDescription>),
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ServiceDescription {
    /// Path to a config file of a tested service.
    pub config_path: String,

    /// Path to compiled modules of a service.
    #[darling(default)]
    pub modules_dir: Option<String>,
}

impl FromMeta for MTestAttributes {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let single_service = ServiceDescription::from_list(items);
        let multiple_services = HashMap::<String, ServiceDescription>::from_list(items);
        match (single_service, multiple_services) {
            (Ok(modules), Err(_)) => Ok(Self::SingleService(modules)),
            (Err(_), Ok(services)) if !services.is_empty() => Ok(Self::MultipleServices(services)),
            (Err(_), Ok(_)) => Err(darling::Error::custom(
                r#"Need to specify "config_path" and "modules_dir" or several named services with these fields "#,
            )),
            (Err(error_single), Err(error_multiple)) => Err(darling::error::Error::multiple(vec![
                error_single,
                error_multiple,
            ])),
            (Ok(_), Ok(_)) => Err(darling::Error::custom(
                "internal sdk error: marine_test attributes are ambiguous",
            )),
        }
    }
}
