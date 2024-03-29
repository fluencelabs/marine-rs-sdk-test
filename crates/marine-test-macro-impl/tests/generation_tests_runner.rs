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

mod utils;

use utils::test_marine_test_token_streams;
use utils::TestServiceDescription;
use utils::test_marine_test_token_streams_multiservice;

#[test]
fn test_empty_func() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/empty_func/marine_test.rs",
        "tests/generation_tests/empty_func/expanded.rs",
        "Config.toml",
    ));
}

#[test]
fn test_mounted_binary() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/mounted_binary/marine_test.rs",
        "tests/generation_tests/mounted_binary/expanded.rs",
        "Config.toml",
    ));
}

#[test]
fn test_multiple_modules() {
    assert!(test_marine_test_token_streams(
        "tests/generation_tests/multiple_modules/marine_test.rs",
        "tests/generation_tests/multiple_modules/expanded.rs",
        "Config.toml",
    ));
}

#[test]
fn test_multiservice_single() {
    let descriptions = vec![TestServiceDescription {
        config_path: "empty_func/Config.toml",
        name: "empty_func",
    }];
    assert!(test_marine_test_token_streams_multiservice(
        "tests/generation_tests/multi-service-single/marine_test.rs",
        "tests/generation_tests/multi-service-single/expanded.rs",
        descriptions
    ));
}

#[test]
fn test_multiservice_multiple() {
    let descriptions = vec![
        TestServiceDescription {
            config_path: "empty_func/Config.toml",
            name: "empty_func",
        },
        TestServiceDescription {
            config_path: "mounted_binary/Config.toml",
            name: "mounted_binary",
        },
    ];
    assert!(test_marine_test_token_streams_multiservice(
        "tests/generation_tests/multi-service-multiple/marine_test.rs",
        "tests/generation_tests/multi-service-multiple/expanded.rs",
        descriptions
    ));
}

#[test]
fn test_multiservice_empty_mod() {
    let descriptions = vec![TestServiceDescription {
        config_path: "Config.toml",
        name: "empty_mod",
    }];
    assert!(test_marine_test_token_streams_multiservice(
        "tests/generation_tests/multi-service-empty_mod/marine_test.rs",
        "tests/generation_tests/multi-service-empty_mod/expanded.rs",
        descriptions
    ));
}
