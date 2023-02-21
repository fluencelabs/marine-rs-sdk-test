# Marine Test Rust SDK
[![crates.io version](https://img.shields.io/crates/v/marine-rs-sdk-test?color=green)](https://crates.io/crates/marine-rs-sdk-test)

This SDK aims to help developers targeting [Marine](https://github.com/fluencelabs/marine) to test their Wasm modules and services because `cargo test` can't run such modules, but it's necessary for testing. To avoid that limitation, the sdk introduces the `#[marine_test]` macro that does much of the heavy lifting to allow developers to use `cargo test` as intended. That is, the `#[marine_test]` macro generates the necessary code to call Marine, one instance per test function, based on the Wasm module and associated configuration file so that the actual test function is run against the Wasm module, not the native code.


## Usage

The core component of the sdk is the `#[marine_test]` macro that can wrap a test function, providing an experience similar to "vanilla" Rust. A wrapped function should receive a special object representing a module interface, let's see an example:
```rust
use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn test(greeting: marine_test_env::greeting::ModuleInterface) {
        let actual = greeting.greeting("John".to_string());
        assert_eq!(actual, "Hi, John");
    }
}
```
This example shows a simple [module](https://fluence.dev/docs/marine-book/quick-start/develop-a-single-module-service) with one export function `greeting` and a test. The test function is wrapped with `#[marine_test]` macro with the specified path to the config file (`Config.toml`), and the directory containing the Wasm module we obtained after compiling our project with the [marine CLI](https://fluence.dev/docs/marine-book/marine-tooling-reference/marine-cli) build command. This macro generates all the necessary glue code to instantiate Marine instance under the hood and call the greeting module loaded into it.

After we have our Wasm module and tests in place, we can proceed with `cargo test`.

In a setup without the Marine test suite, the `greeting` function will be compiled to native and then test natively, comparingly with the suite it will be compiled to Wasm, loaded into Marine, and only then called as a Wasm module.

More details can be found in [this chapter](https://fluence.dev/docs/marine-book/marine-rust-sdk/testing-and-debugging/) of the Marine book.


## Documentation

- [Marine Book](https://fluence.dev/docs/marine-book/introduction)
- [Marine Examples](https://github.com/fluencelabs/examples/tree/main/marine-examples)
- [Quickstart](https://fluence.dev/docs/marine-book/quick-start/)


## Repository structure

- **[crates](./crates)**
    - [macro-build-rs-generator](./crates/macro-build-rs-generator) - generator of `build.rs` file intended to provide IDE support for generated glue code
    - [marine-test-macro-impl](./crates/marine-test-macro-impl) - actual implementation of the `#[marine_test]` macro
    - [marine-test-macro](./crates/marine-test-macro) - proc-macro crate for the `#[marine_test]` macro
- **[src](./src)** - reexports all necessary things intended to use by end user


## Support

Please, [file an issue](https://github.com/fluencelabs/marine-rs-sdk-test/issues) if you find a bug. You can also contact us at [Discord](https://discord.com/invite/5qSnPZKh7u) or [Telegram](https://t.me/fluence_project). We will do our best to resolve the issue ASAP.


## Contributing

Any interested person is welcome to contribute to the project. Please, make sure you read and follow some basic [rules](./CONTRIBUTING.md).


## License

All software code is copyright (c) Fluence Labs, Inc. under the [Apache-2.0](./LICENSE) license.

