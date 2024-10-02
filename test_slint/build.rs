fn main() {
    // slint_build::compile("main.slint").unwrap();

    let config = slint_build::CompilerConfiguration::new().with_style("material-dark".into());
    slint_build::compile_with_config("main.slint", config).unwrap();
}
