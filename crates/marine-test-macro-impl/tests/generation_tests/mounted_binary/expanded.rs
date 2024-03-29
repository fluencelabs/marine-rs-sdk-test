#[test]
fn test() {
    #[allow(unused)]
    #[allow(non_snake_case)]
    #[allow(unused_imports)]
    pub mod marine_test_env {
        pub mod greeting {
            #[derive(
                Clone,
                Debug,
                marine_rs_sdk_test :: internal :: serde :: Serialize,
                marine_rs_sdk_test :: internal :: serde :: Deserialize,
            )]
            #[serde(crate = "marine_rs_sdk_test::internal::serde")]
            pub struct CallParameters {
                pub init_peer_id: String,
                pub service_id: String,
                pub service_creator_peer_id: String,
                pub host_id: String,
                pub particle_id: String,
                pub tetraplets: Vec<Vec<SecurityTetraplet>>,
            }
            #[derive(
                Clone,
                Debug,
                marine_rs_sdk_test :: internal :: serde :: Serialize,
                marine_rs_sdk_test :: internal :: serde :: Deserialize,
            )]
            #[serde(crate = "marine_rs_sdk_test::internal::serde")]
            pub struct MountedBinaryResult {
                pub ret_code: i32,
                pub error: String,
                pub stdout: Vec<u8>,
                pub stderr: Vec<u8>,
            }
            #[derive(
                Clone,
                Debug,
                marine_rs_sdk_test :: internal :: serde :: Serialize,
                marine_rs_sdk_test :: internal :: serde :: Deserialize,
            )]
            #[serde(crate = "marine_rs_sdk_test::internal::serde")]
            pub struct MountedBinaryStringResult {
                pub ret_code: i32,
                pub error: String,
                pub stdout: String,
                pub stderr: String,
            }
            #[derive(
                Clone,
                Debug,
                marine_rs_sdk_test :: internal :: serde :: Serialize,
                marine_rs_sdk_test :: internal :: serde :: Deserialize,
            )]
            #[serde(crate = "marine_rs_sdk_test::internal::serde")]
            pub struct SecurityTetraplet {
                pub peer_pk: String,
                pub service_id: String,
                pub function_name: String,
                pub json_path: String,
            }
            pub struct ModuleInterface {
                marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>>,
            }
            impl ModuleInterface {
                pub fn new(
                    marine: std::rc::Rc<
                        std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                    >,
                ) -> Self {
                    Self { marine }
                }
            }
            impl ModuleInterface {
                pub fn download(&mut self, url: String) -> String {
                    let arguments = marine_rs_sdk_test::internal::serde_json::json!([url]);
                    let result = self
                        .marine
                        .as_ref()
                        .borrow_mut()
                        .call_module("greeting", "download", arguments, <_>::default())
                        .expect("call to Marine failed");
                    let result: String =
                        marine_rs_sdk_test::internal::serde_json::from_value(result)
                            .expect("the default deserializer shouldn't fail");
                    result
                }
                pub fn download_cp(
                    &mut self,
                    url: String,
                    cp: marine_rs_sdk_test::CallParameters,
                ) -> String {
                    let arguments = marine_rs_sdk_test::internal::serde_json::json!([url]);
                    let result = self
                        .marine
                        .as_ref()
                        .borrow_mut()
                        .call_module("greeting", "download", arguments, cp)
                        .expect("call to Marine failed");
                    let result: String =
                        marine_rs_sdk_test::internal::serde_json::from_value(result)
                            .expect("the default deserializer shouldn't fail");
                    result
                }
            }
        }
    }
    let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();
    let mut module_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut file_path = std::path::Path::new("tests/generation_tests/mounted_binary").components();
    let mut truncated_file_path = Vec::new();
    loop {
        if module_path.ends_with(file_path.as_path()) {
            break;
        }
        let (file_path_, remainder) = match file_path.next_back().and_then(|p| match p {
            std::path::Component::Normal(_)
            | std::path::Component::CurDir
            | std::path::Component::ParentDir => Some((file_path, p)),
            _ => None,
        }) {
            Some(t) => t,
            None => break,
        };
        file_path = file_path_;
        truncated_file_path.push(remainder);
    }
    for path in truncated_file_path.iter().rev() {
        module_path.push(path);
    }
    let config_path = module_path.join("Config.toml");
    let mut __m_generated_marine_config = marine_rs_sdk_test::internal::TomlAppServiceConfig::load(
        &config_path,
    )
    .unwrap_or_else(|e| {
        panic!(
            "app service config located at `{:?}` can't be loaded: {}",
            config_path, e
        )
    });
    __m_generated_marine_config.toml_marine_config.base_path = config_path
        .parent()
        .map(std::path::PathBuf::from)
        .unwrap_or_default();
    let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(
        __m_generated_marine_config,
        service_id,
        std::collections::HashMap::new(),
    )
    .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
    let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
    let mut greeting = marine_test_env::greeting::ModuleInterface::new(marine.clone());
    fn test_func(greeting: marine_test_env::greeting::ModuleInterface) {
        let mut greeting = greeting;
        {
            let _ = greeting.download("duckduckgo.com");
        }
    }
    test_func(greeting)
}
