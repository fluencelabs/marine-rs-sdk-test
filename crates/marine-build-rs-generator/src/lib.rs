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

#![doc(html_root_url = "https://docs.rs/marine-build-rs-generator/0.8.1")]
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
#![recursion_limit = "1024"]

use marine_test_macro_impl::generate_marine_test_env_impl;
pub use marine_test_macro_impl::ServiceDescription;

use std::path::{PathBuf, Path};
use std::{fs, env};

pub fn generate_marine_test_env(
    services: impl IntoIterator<Item = (String, ServiceDescription)>,
    filename: &str,
    build_rs_file_path: &str,
) {
    // build_rs_file_path expected to be obtained from file!() macro, which returns path with filename,
    // but underlying code expects path without filename, so we are removing last part
    let mut build_rs_file_path = PathBuf::from(build_rs_file_path);
    let _ = build_rs_file_path.pop();

    match generate_marine_test_env_impl(services, &build_rs_file_path) {
        Ok(stream) => {
            let out_dir = env::var_os("OUT_DIR")
                .expect("cannot write marine_test_env: OUT_DIR env var must be set");
            let dest_path = Path::new(&out_dir).join(filename);

            match fs::write(dest_path, stream.to_string()) {
                Ok(result) => result,
                Err(e) => {
                    std::panic::panic_any(format!("cannot write marine_test_env on disk: {}", e))
                }
            }
        }
        Err(error) => std::panic::panic_any(format!("marine_test_env generation error: {}", error)),
    }
}

#[macro_export]
macro_rules! include_test_env {
    ($filename:expr) => {
        include!(concat!(env!("OUT_DIR"), $filename));
    };
}
