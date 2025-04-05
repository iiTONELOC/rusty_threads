use std::{env, fs, path::PathBuf};

fn main() {
    // Output directory provided by Cargo
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Missing OUT_DIR"));

    // === Generate bindings for THREADS ===
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h") // wrapper that includes THREADS headers
        .clang_arg("-Iinclude") // point to header directory
        .clang_arg("-fms-extensions") // MSVC compatibility
        .clang_arg("-fdeclspec") // For __declspec macros in headers
        // Explicitly list what we want bindings for
        // ensures we don't build bindings for all of windows.h,
        // as it is included in the THREADS headers
        .allowlist_function("context_initialize")
        .allowlist_function("context_switch")
        .allowlist_function("context_stop")
        .allowlist_function("get_psr")
        .allowlist_function("set_psr")
        .allowlist_function("system_clock")
        .allowlist_function("get_interrupt_handlers")
        .allowlist_function("get_system_call_vector")
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // Let cargo track file changes
        .generate()
        .expect("Unable to generate bindings for THREADSLib");

    // Write bindings to $OUT_DIR/thread_bindings.rs
    bindings
        .write_to_file(out_path.join("thread_bindings.rs"))
        .expect("Couldn't write THREADSLib bindings!");

    // === Link .lib files ===
    println!("cargo:rustc-link-search=native=lib/"); // path to .lib/ directory
    println!("cargo:rustc-link-lib=dylib=THREADS"); // name of .lib file (without 'lib' prefix or extension)


     // === Copy DLL to output folder ===
     let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into()))
     .join("debug");

    let dll_src = PathBuf::from("lib").join("THREADS.dll");
    let dll_dst = target_dir.join("THREADS.dll");

    if let Err(e) = fs::copy(&dll_src, &dll_dst) {
        println!("cargo:warning=Could not copy THREADS.dll: {e}");
    }
}
