# Changelog

## [0.12.1](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.12.0...marine-rs-sdk-test-v0.12.1) (2023-12-20)


### Bug Fixes

* allow unused imports in marine-test-env ([#111](https://github.com/fluencelabs/marine-rs-sdk-test/issues/111)) ([05321fe](https://github.com/fluencelabs/marine-rs-sdk-test/commit/05321fe7b0e09fd95f2b190e08a10fb278f83779))

## [0.12.0](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.11.1...marine-rs-sdk-test-v0.12.0) (2023-12-14)


### ⚠ BREAKING CHANGES

* **deps:** update to marine runtime with memory limits ([#98](https://github.com/fluencelabs/marine-rs-sdk-test/issues/98))

### Features

* **deps:** update to marine runtime with memory limits ([#98](https://github.com/fluencelabs/marine-rs-sdk-test/issues/98)) ([f772635](https://github.com/fluencelabs/marine-rs-sdk-test/commit/f772635ccc24f81d1e82da733324e98eebd454c7))

## [0.11.1](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.11.0...marine-rs-sdk-test-v0.11.1) (2023-11-21)


### Features

* **deps:** update rust crate proc-macro2 to 1.0.69 [[#84](https://github.com/fluencelabs/marine-rs-sdk-test/issues/84)](https://github.com/fluencelabs/marine-rs-sdk-test/pull/84) ([fd6b6c0](https://github.com/fluencelabs/marine-rs-sdk-test/commit/fd6b6c0d52e787f5c443c826e3573b4b088d6271))

## [0.11.0](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.10.2...marine-rs-sdk-test-v0.11.0) (2023-10-24)


### ⚠ BREAKING CHANGES

* **deps:** update fluence-app-service to 0.29.0 ([#99](https://github.com/fluencelabs/marine-rs-sdk-test/issues/99))

### Features

* **deps:** update fluence-app-service to 0.29.0 ([#99](https://github.com/fluencelabs/marine-rs-sdk-test/issues/99)) ([c870fa7](https://github.com/fluencelabs/marine-rs-sdk-test/commit/c870fa7716c77f8ff68b031275fd999726ef1ffd))

## [0.10.2](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.10.1...marine-rs-sdk-test-v0.10.2) (2023-08-09)


### Features

* update marine runtime ([#93](https://github.com/fluencelabs/marine-rs-sdk-test/issues/93)) ([749429d](https://github.com/fluencelabs/marine-rs-sdk-test/commit/749429ddcbc890808a20235530ddd32e61d4dd5d))

## [0.10.1](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.10.0...marine-rs-sdk-test-v0.10.1) (2023-08-07)


### Features

* update marine runtime ([#91](https://github.com/fluencelabs/marine-rs-sdk-test/issues/91)) ([9cd7f83](https://github.com/fluencelabs/marine-rs-sdk-test/commit/9cd7f834cb6648973a2cf49efb88e5df594930c2))

## [0.10.0](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.9.1...marine-rs-sdk-test-v0.10.0) (2023-05-05)


### ⚠ BREAKING CHANGES

* use only config to load modules, deprecate modules_dir parameter ([#78](https://github.com/fluencelabs/marine-rs-sdk-test/issues/78))

### Features

* use only config to load modules, deprecate modules_dir parameter ([#78](https://github.com/fluencelabs/marine-rs-sdk-test/issues/78)) ([b57fcc4](https://github.com/fluencelabs/marine-rs-sdk-test/commit/b57fcc4ab1b06396e3c3f28a8aa3a7157c98a71e))

## [0.9.1](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.9.0...marine-rs-sdk-test-v0.9.1) (2023-04-13)


### Bug Fixes

* remove warnings in marine_test_env + update marine ([#73](https://github.com/fluencelabs/marine-rs-sdk-test/issues/73)) ([c055e6d](https://github.com/fluencelabs/marine-rs-sdk-test/commit/c055e6de90d139e4dffad258af7512ae50483d91))

## [0.9.0](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.8.2...marine-rs-sdk-test-v0.9.0) (2023-03-17)


### ⚠ BREAKING CHANGES

* **runtime:** update marine ([#64](https://github.com/fluencelabs/marine-rs-sdk-test/issues/64))

### Features

* **runtime:** update marine ([#64](https://github.com/fluencelabs/marine-rs-sdk-test/issues/64)) ([68e86c7](https://github.com/fluencelabs/marine-rs-sdk-test/commit/68e86c7bdb1ddc69d562bbd34b3f8d94de902bd5))

## [0.8.2](https://github.com/fluencelabs/marine-rs-sdk-test/compare/marine-rs-sdk-test-v0.8.1...marine-rs-sdk-test-v0.8.2) (2023-02-21)


### Bug Fixes

* **deps:** update rust crate fluence-app-service to 0.23.1 ([#50](https://github.com/fluencelabs/marine-rs-sdk-test/issues/50)) ([1167214](https://github.com/fluencelabs/marine-rs-sdk-test/commit/11672142621d79293ec17f6ae56e4d48fc4ec835))
* **deps:** update rust crate marine-it-parser to 0.11.1 ([#41](https://github.com/fluencelabs/marine-rs-sdk-test/issues/41)) ([0baeb86](https://github.com/fluencelabs/marine-rs-sdk-test/commit/0baeb863cc066cb5efb72e28986aac1408e08c9b))

## Version 0.4.0 (2021-10-18)
[PR 10](https://github.com/fluencelabs/marine-rs-sdk-test/pull/10):
- multi-service `marine_test` can now be applied to a `mod` instead of a `fn`
- added `generate_marine_test_env` function for generating `marine_test_env` in `build.rs`. It generates the same `marine_test_env` as the multi-service `marine_test` does and has similar interface. This will allow IDE support.
- added `include_test_env!` declarative macro for including generated `marine_test_env` in the project

## Version 0.3.0 (2021-10-04)
[PR 61](https://github.com/fluencelabs/marine-rs-sdk/pull/61):

Implemented the first part of [Issue 57](https://github.com/fluencelabs/marine-rs-sdk/issues/57): `marine_test` now can take several named services in attributes, then define interface to the services in `marine_test_env`.

## Version 0.2.0 (2021-09-01)
[PR 54](https://github.com/fluencelabs/marine-rs-sdk/pull/54):
- previously test function accessed module interfaces through externally defined variables, now module interfaces are passed as arguments.
- introduced generated module `marine_test_env` which provides interface for generated structs and functions.
