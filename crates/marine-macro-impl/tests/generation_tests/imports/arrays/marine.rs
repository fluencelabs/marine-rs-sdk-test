#[link(wasm_import_module = "test")]
extern "C" {
    pub fn inner_arrays_1(arg: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>>;
}
