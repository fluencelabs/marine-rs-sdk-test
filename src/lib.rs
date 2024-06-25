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

#![doc(html_root_url = "https://docs.rs/marine-rs-sdk-test/0.8.1")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

pub use marine_test_macro::marine_test;
pub use marine_test_macro::fce_test;

pub use marine_build_rs_generator::generate_marine_test_env;
pub use marine_build_rs_generator::ServiceDescription;
pub use marine_build_rs_generator::include_test_env;

pub use fluence_app_service::CallParameters;
pub use fluence_app_service::ParticleParameters;
pub use fluence_app_service::SecurityTetraplet;

/// These API functions are intended for internal usage in generated code.
/// Normally, you shouldn't use them.
pub mod internal {
    pub use fluence_app_service::AppService;
    pub use fluence_app_service::TomlAppServiceConfig;

    pub use serde;
    pub use serde_json;

    pub use uuid::Uuid;
}
