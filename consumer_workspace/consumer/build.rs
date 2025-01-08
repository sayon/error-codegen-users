fn main() {
    println!("cargo:rerun-if-changed=resources/errors.json");
    zksync_error_codegen::default_load_and_generate("provider_root")
}
