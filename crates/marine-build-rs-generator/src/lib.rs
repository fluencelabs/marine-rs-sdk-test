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

#![doc(html_root_url = "https://docs.rs/marine-build-rs-generator/0.7.2")]
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
