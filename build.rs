use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Missing OUT_DIR"));

    // === Generate bindings for THREADS ===
    let bindings = bindgen::Builder::default()
        .header(
            manifest_dir
                .join("include")
                .join("wrapper.h")
                .to_str()
                .unwrap(),
        )
        .clang_arg(format!(
            "-I{}",
            manifest_dir.join("include").to_str().unwrap()
        ))
        .clang_arg("-fms-extensions")
        .clang_arg("-fdeclspec")
        .allowlist_function("context_initialize")
        .allowlist_function("context_switch")
        .allowlist_function("context_stop")
        .allowlist_function("get_psr")
        .allowlist_function("set_psr")
        .allowlist_function("system_clock")
        .allowlist_function("get_interrupt_handlers")
        .allowlist_function("device_initialize")
        .allowlist_function("device_handle")
        .allowlist_function("device_control")
        .allowlist_function("set_debug_level")
        .allowlist_function("console_output")
        .allowlist_function("stop")
        .allowlist_function("bootstrap")
        .allowlist_type("device_control_block_t")
        .allowlist_type("system_call_arguments_t")
        .allowlist_type("device_type_t")
        .allowlist_type("process_entrypoint_t")
        .allowlist_type("interrupt_handler_t")
        .allowlist_type("system_call_handler_t")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for THREADSLib");

    bindings
        .write_to_file(out_path.join("thread_bindings.rs"))
        .expect("Couldn't write THREADSLib bindings!");

    // === Link .lib files ===
    let lib_path = manifest_dir.join("lib");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=THREADS");
    println!("cargo:rustc-link-lib=static=ThreadsMain");

    // === Copy DLL to output folder ===
let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into()))
.join("debug");

// ensure the target directory exists
if let Err(e) = std::fs::create_dir_all(&target_dir) {
println!("cargo:warning=Could not create output dir: {e}");
}

let dll_src = manifest_dir.join("lib").join("THREADS.dll");
let dll_dst = target_dir.join("THREADS.dll");

if dll_src.exists() {
if let Err(e) = fs::copy(&dll_src, &dll_dst) {
    println!("cargo:warning=Could not copy THREADS.dll: {e}");
}
} else {
println!("cargo:warning=THREADS.dll not found at {:?}", dll_src);
}

}
