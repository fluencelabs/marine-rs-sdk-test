use marine_test_macro_impl::generate_marine_test_env_impl;
use std::collections::HashMap;

pub use marine_test_macro_impl::ServiceDescription;
use std::path::{PathBuf, Path};
use std::{fs, env};

pub fn generate_marine_test_env(
    services: &[(String, ServiceDescription)],
    filename: &str,
    build_rs_file_path: &str,
) {
    let services = services
        .iter()
        .cloned()
        .collect::<HashMap<String, ServiceDescription>>();

    let mut build_rs_file_path = PathBuf::from(build_rs_file_path);
    let _ = build_rs_file_path.pop();

    match generate_marine_test_env_impl(services, &build_rs_file_path) {
        Ok(stream) => {
            let out_dir = env::var_os("OUT_DIR")
                .expect("cannot write marine_test_env: OUT_DIR env var must be set");
            let dest_path = Path::new(&out_dir).join(filename);

            let result = fs::write(dest_path, stream.to_string());
            if let Err(e) = result {
                std::panic::panic_any(format!(
                    "cannot write marine_test_env on disk: {}",
                    e.to_string()
                ));
            }
        }
        Err(error) => std::panic::panic_any(format!(
            "marine_test_env generation error: {}",
            error.to_string()
        )),
    }
}

#[macro_export]
macro_rules! include_test_env {
    ($filename:expr) => { include!(concat!(env!("OUT_DIR"), $filename));}
}
