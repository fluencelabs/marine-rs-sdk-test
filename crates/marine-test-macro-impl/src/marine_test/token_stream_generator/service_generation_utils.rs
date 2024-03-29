use crate::TResult;
use crate::TestGeneratorError;
use crate::marine_test::config_utils::Module;
use crate::marine_test::utils::new_ident;
use crate::marine_test::{modules_linker, config_utils};
use crate::marine_test::modules_linker::{LinkedModule, UseDescription};
use super::service_generator::{ProcessedService, get_facace};

use proc_macro2::TokenStream;
use quote::quote;

use std::path::Path;
use itertools::Itertools;

pub(crate) fn generate_app_service_ctor(
    config_path: &str,
    test_file_path: &Path,
) -> TResult<TokenStream> {
    let test_file_path = test_file_path
        .to_str()
        .ok_or_else(|| TestGeneratorError::InvalidUTF8Path(test_file_path.to_path_buf()))?;

    let service_ctor = quote! {
        let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();
        let mut module_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut file_path = std::path::Path::new(#test_file_path).components();

        let mut truncated_file_path = Vec::new();
        loop {
            if module_path.ends_with(file_path.as_path()) {
                break;
            }

            let (file_path_, remainder) = match file_path.next_back().and_then(|p| match p {
                std::path::Component::Normal(_) | std::path::Component::CurDir | std::path::Component::ParentDir => Some((file_path, p)),
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

        let config_path = module_path.join(#config_path);

        let mut __m_generated_marine_config = marine_rs_sdk_test::internal::TomlAppServiceConfig::load(&config_path,)
            .unwrap_or_else(|e| {
                panic!("app service config located at `{:?}` can't be loaded: {}", config_path, e)
            } );
        __m_generated_marine_config.toml_marine_config.base_path = config_path
        .parent()
        .map(std::path::PathBuf::from)
        .unwrap_or_default();

        let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(__m_generated_marine_config, service_id, std::collections::HashMap::new(),)
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));

        let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
    };

    Ok(service_ctor)
}

pub(super) fn generate_service_definition(
    service: &ProcessedService,
    linked_facade: &LinkedModule<'_>,
    file_path_for_app_service: &Path,
) -> TResult<TokenStream> {
    let modules = config_utils::collect_modules(&service.config)?;
    let linked_modules = modules_linker::link_modules(
        modules
            .iter()
            .map(|module| (module.name, &module.interface)),
    )?;

    let service_mod = new_ident(&service.name)?;
    let module_definitions = super::generate_module_definitions(modules.iter(), &linked_modules)?;

    let facade = get_facace(&modules)?;

    let facade_interface = super::methods_generator::generate_facade_methods(
        facade.interface.function_signatures.iter(),
        &facade.interface.record_types,
    )?;

    let facade_override =
        super::generate_module_definition(facade, linked_facade, service_import_generator)?;
    let facade_override_ident = new_ident("__facade_override")?;
    let facade_structs = generate_facade_structs(facade, &facade_override_ident)?;

    let app_service_ctor =
        generate_app_service_ctor(&service.config_path, file_path_for_app_service)?;
    let modules_type = generate_modules_type(&modules)?;

    let service_definition = quote! {
        pub mod #service_mod {
            pub mod modules {
                #(#module_definitions)*
            }

            pub mod #facade_override_ident {
                #facade_override
            }

            #(#facade_structs)*

            #modules_type

            pub struct ServiceInterface {
                pub modules: __GeneratedModules,
                __facade: #facade_override_ident::ModuleInterface,
                marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >
            }

            impl ServiceInterface {
                pub fn new() -> Self {
                    #app_service_ctor
                    let modules = __GeneratedModules::new(marine.clone());
                    let __facade = #facade_override_ident::ModuleInterface::new(marine.clone());
                    Self {
                        marine,
                        modules,
                        __facade
                    }
                }

                #(#facade_interface)*
            }
        }
    };

    Ok(service_definition)
}

fn service_import_generator(info: &UseDescription<'_>) -> TResult<TokenStream> {
    let from_module_ident = new_ident(info.from)?;
    let record_name_ident = new_ident(info.name)?;
    Ok(quote! {pub use super::super::#from_module_ident::#record_name_ident;})
}

fn generate_facade_structs(
    module: &Module<'_>,
    module_name: &syn::Ident,
) -> TResult<Vec<TokenStream>> {
    module
        .interface
        .record_types
        .iter()
        .sorted_by_key(|(_, record)| &record.name)
        .map(|(_, record)| -> TResult<TokenStream> {
            let record_name = new_ident(&record.name)?;
            let result = quote! {pub use #module_name::#record_name;};
            Ok(result)
        })
        .collect::<TResult<Vec<TokenStream>>>()
}

fn generate_modules_type(module_interfaces: &[Module<'_>]) -> TResult<TokenStream> {
    let fields = module_interfaces
        .iter()
        .map(|module| -> TResult<TokenStream> {
            let name = new_ident(module.name)?;
            Ok(quote! {pub #name: modules::#name::ModuleInterface})
        })
        .collect::<TResult<Vec<TokenStream>>>()?;

    let ctors = module_interfaces
        .iter()
        .map(|module| -> TResult<TokenStream> {
            let name = new_ident(module.name)?;
            Ok(quote! {#name: modules::#name::ModuleInterface::new(marine.clone())})
        })
        .collect::<TResult<Vec<TokenStream>>>()?;

    let ty = quote! {
        pub struct __GeneratedModules {
            #(#fields,)*
        }

        impl __GeneratedModules {
            fn new(marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >) -> Self {
                Self {
                    #(#ctors,)*
                }
            }
        }
    };

    Ok(ty)
}
