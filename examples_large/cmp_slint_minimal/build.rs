use std::{collections::HashMap, env::var_os, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_paths = HashMap::from([(
        "material".to_string(),
        manifest_dir.join("src/config_slint/app/components/material/material.slint"),
    )]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config("src/config_slint/app/global/main.slint", config).unwrap();
}
