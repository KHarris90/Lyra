#[test]
fn compile_smoke() {
    let out = compiler::compile("");
    // For now, just ensure pipeline returns an output
    let _ = out.module;
}
