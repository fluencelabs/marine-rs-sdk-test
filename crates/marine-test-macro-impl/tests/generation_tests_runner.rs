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
