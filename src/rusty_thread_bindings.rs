// rusty_thread_bindings.rs

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, improper_ctypes)]
mod rusty_bindings {
    include!(concat!(env!("OUT_DIR"), "/thread_bindings.rs"));
}


// Re-export the bindings for use in other modules
// aliases are used to avoid name conflicts, we will use the non-aliased names
// as the front-facing Rust API, which will be defined in the rusty_wrapper module
// and exposed through the library interface.
#[allow(unused_imports)]
pub use rusty_bindings::{
    device_type_t,
    system_call_handler_t,
    device_control_block_t,
    system_call_arguments_t,
    process_entrypoint_t,
    interrupt_handler_t,
    console_output as c_console_output, context_initialize as c_context_initialize,
    context_stop as c_context_stop, context_switch as c_context_switch,
    device_control as c_device_control, device_handle as c_device_handle,
    device_initialize as c_device_initialize, get_interrupt_handlers as c_get_interrupt_handlers,
    get_psr as c_get_psr, get_system_call_vector as c_get_system_call_vector,
    set_debug_level as c_set_debug_level, set_psr as c_set_psr, stop as c_stop,
    system_clock as c_system_clock,
};


// test the bindings
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_linkage() {
        // Function pointers: check that they exist and are not null
        assert!((c_console_output as usize) != 0);
        assert!((c_context_initialize as usize) != 0);
        assert!((c_context_stop as usize) != 0);
        assert!((c_context_switch as usize) != 0);
        assert!((c_device_control as usize) != 0);
        assert!((c_device_handle as usize) != 0);
        assert!((c_device_initialize as usize) != 0);
        assert!((c_get_interrupt_handlers as usize) != 0);
        assert!((c_get_psr as usize) != 0);
        assert!((c_get_system_call_vector as usize) != 0);
        assert!((c_set_debug_level as usize) != 0);
        assert!((c_set_psr as usize) != 0);
        assert!((c_stop as usize) != 0);
        assert!((c_system_clock as usize) != 0);

       // Type sizes: check that they are not zero
        assert!(core::mem::size_of::<device_type_t>() > 0);
        assert!(core::mem::size_of::<system_call_handler_t>() > 0);
        assert!(core::mem::size_of::<device_control_block_t>() > 0);
        assert!(core::mem::size_of::<system_call_arguments_t>() > 0);
        assert!(core::mem::size_of::<process_entrypoint_t>() > 0);
        assert!(core::mem::size_of::<interrupt_handler_t>() > 0);

        // Type Alignments: check that they are not zero
        assert!(core::mem::align_of::<device_type_t>() > 0);
        assert!(core::mem::align_of::<system_call_handler_t>() > 0);
        assert!(core::mem::align_of::<device_control_block_t>() > 0);
        assert!(core::mem::align_of::<system_call_arguments_t>() > 0);
        assert!(core::mem::align_of::<process_entrypoint_t>() > 0);
        assert!(core::mem::align_of::<interrupt_handler_t>() > 0);

    }


}
