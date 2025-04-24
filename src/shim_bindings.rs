#[link(name = "console_output_shim")] // ← match .compile("console_output_shim") from build.rs
unsafe extern "C" {
    pub fn console_output_str(debug: bool, msg: *const std::os::raw::c_char);
}