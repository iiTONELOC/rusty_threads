use std::ffi::CString;
use crate::rusty_thread_bindings::*;
use crate::constants::THREADS_INTERRUPT_HANDLER_COUNT;


/// Initializes a new process context.
///
/// This function sets up a new execution context within the
/// THREADS kernel session. It requires an entry point function (`entry_point`)
/// which acts as the starting point for the new context.
///
/// In the kernel, each process has its own context. The returned context pointer
/// can later be passed to `context_switch()` to transfer control.
///
/// The THREADS library manages stack allocation internally, but the caller
/// must specify the stack size when initializing the context.
///
/// # Arguments
///
/// * `entry_point` – A function pointer that represents the start of execution for the context.
/// * `stack_size` – The size of the stack in bytes (minimum 8192 recommended).
/// * `args` – A raw pointer to argument data passed to the context entry function.
///
/// # Returns
///
/// A raw pointer to the created context, which can be passed to other THREADS functions.
///
/// # Safety
///
/// - The `entry_point` function must follow the (CDECL) calling convention.
/// - The `args` pointer must be valid and aligned properly.
/// - Misuse can result in undefined behavior, stack corruption, or crashes.
pub unsafe fn context_initialize(
    entry: extern "C" fn(*mut core::ffi::c_void) -> i32,
    stack_size: i32,
    args: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    unsafe { c_context_initialize(Some(entry), stack_size, args) }
}

/// Switches execution to a new context.
///
/// This function transfers execution from the current context to another
/// that has already been initialized with `context_initialize()`. The new
/// context is identified by the `next_context` pointer — the same value
/// returned by `context_initialize()`.
///
/// # Arguments
///
/// * `next_context` – A raw pointer to the context that should be scheduled next.
///
/// # Returns
///
/// * `true` if the switch succeeded.
/// * `false` if an error occurred during the switch.
///
/// # Safety
///
/// - `next_context` must be a valid pointer to a context created by `context_initialize()`.
/// - If the pointer is invalid or null, the behavior is undefined.
pub unsafe fn context_switch(next: *mut core::ffi::c_void) -> bool {
    unsafe { c_context_switch(next) }
}

/// Stops and deallocates a process context.
///
/// This function halts execution of the specified context and frees its internal
/// resources. It should be used when the kernel is shutting down a process
/// and the associated context is no longer needed.
///
/// The `context` pointer must be one that was previously returned by
/// `context_initialize()`.
///
/// # Arguments
///
/// * `context` – A raw pointer to the context to be stopped.
///
/// # Returns
///
/// This function does not return a value.
///
/// # Safety
///
/// - `context` must be a valid pointer created by `context_initialize()`.
/// - The pointer must not have already been freed or used after this call.
/// - Behavior is undefined if the pointer is invalid.
pub unsafe fn context_stop(ctx: *mut core::ffi::c_void) {
    unsafe { c_context_stop(ctx) }
}

/// Retrieves the current value of the Processor Status Register (PSR).
///
/// The PSR is a bitmask that describes the current processor mode and
/// interrupt state. The returned value contains one or more of the following
/// bit flags:
///
/// - `PSR_INTERRUPTS` (1): Interrupts are enabled.
/// - `PSR_KERNEL_MODE` (2): Processor is in kernel mode.
/// - `PSR_IRQ_MODE` (4): IRQ handling is active.
///
/// # Returns
///
/// A `u32` bitmask representing the current PSR state.
///
/// # Example
///
/// ```ignore
/// let psr = rusty_threads::get_psr();
/// if psr & rusty_threads::PSR_KERNEL_MODE != 0 {
///     println!("Running in kernel mode.");
/// }
/// ```
pub fn get_psr() -> u32 {
    unsafe { c_get_psr() }
}

/// Sets the value of the Processor Status Register (PSR).
///
/// The PSR is a bitmask that defines the processor's current execution mode
/// and interrupt configuration. Valid bit flags include:
///
/// - `PSR_INTERRUPTS` (1): Enables/disables interrupts.
/// - `PSR_KERNEL_MODE` (2): Switches to kernel mode.
/// - `PSR_IRQ_MODE` (4): Enables IRQ handling mode.
///
/// This function overwrites the PSR entirely with the specified value.
///
/// # Arguments
///
/// * `psr` – A 32-bit unsigned integer containing the new PSR bitmask value.
///
/// # Safety
///
/// - Setting the PSR incorrectly may disable interrupts or corrupt system mode state.
/// - This function must be used with care in kernel or system-level code.
///
/// # Example
///
/// ```ignore
/// // Enable interrupts and enter kernel mode
/// rusty_threads::set_psr(rusty_threads::PSR_INTERRUPTS | rusty_threads::PSR_KERNEL_MODE);
/// ```
pub fn set_psr(psr: u32) {
    unsafe { c_set_psr(psr) }
}

/// Returns the current value of the system clock in microseconds.
///
/// The system clock is a high-resolution counter that begins at zero when the
/// THREADS kernel is initialized. It increases monotonically and provides
/// precise timing for events, scheduling, and performance measurement
/// within the simulated operating system.
///
/// # Returns
///
/// A `u32` representing the number of microseconds since kernel startup.
///
/// # Example
///
/// ```ignore
/// let start = rusty_threads::system_clock();
/// // ... do something time-sensitive ...
/// let end = rusty_threads::system_clock();
/// let elapsed = end - start;
/// println!("Elapsed time: {} μs", elapsed);
/// ```
pub fn system_clock() -> u32 {
    unsafe { c_system_clock() }
}


/// Initializes the specified device by name.
///
/// This function calls into the THREADS kernel to initialize a device,
/// returning a handle used for subsequent operations (e.g., `device_control`, `device_handle`).
///
/// # Arguments
///
/// * `device_name` - A string identifying the device (e.g., `"disk0"`, `"term1"`).
///
/// # Returns
///
/// A device handle (`u32`) if the initialization succeeds.  
/// Returns `u32::MAX` (equivalent to `-1` in C) if the device could not be initialized.
///
/// # Example
///
/// ``` ignore
/// let handle = rusty_threads::device_initialize("disk0").expect("Failed to init device");
/// ```
///
/// # Errors
///
/// Returns `None` if the device name is invalid or the device could not be initialized.
pub fn device_initialize(device_name: &str) -> Option<u32> {
    // Convert the device name to a C-compatible string, ensuring it doesn't contain null bytes.
    let c_str = CString::new(device_name).expect("Device name contains null byte");
    // Call the C function to initialize the device.
    // The function returns a handle or u32::MAX aka (-1) on failure.
    let handle = unsafe { c_device_initialize(c_str.as_ptr() as *mut _) };
    // Check if the handle is valid (not u32::MAX), or -1.
    if handle == u32::MAX {
        None
    } else {
        Some(handle)
    }
}

/// Retrieves the handle of an initialized I/O device by name.
///
/// This function returns the device handle associated with the given device name,
/// allowing the caller to interact with the device using `device_control()` or other
/// THREADS functions.
///
/// # Arguments
///
/// * `device_name` – A string representing the device name (e.g., `"disk0"`, `"term1"`).
///
/// # Returns
///
/// * `Some(handle)` – If the device exists and is valid.
/// * `None` – If the device name is invalid or not recognized by the kernel.
///
/// # Example
///
/// ``` ignore
/// let handle = rusty_threads::device_handle("disk0").expect("Device not found");
/// ```
pub fn device_handle(device_name: &str) -> Option<u32> {
    let c_str = CString::new(device_name).expect("Device name contains null byte");
    let handle = unsafe { c_device_handle(c_str.as_ptr() as *mut _) };
    if handle == u32::MAX {
        None
    } else {
        Some(handle)
    }
}

/// Issues a device control operation for a specified I/O device.
///
/// This function sends a control command to a device using a [`device_control_block_t`] structure,
/// which abstracts over the device-specific command, data buffers, and argument values.
///
/// # Arguments
///
/// * `device_name` – The name of the device (e.g., `"disk0"`, `"term1"`).
/// * `control_block` – A fully populated [`device_control_block_t`] representing the I/O operation.
///
/// # Returns
///
/// * `Some(handle)` – If the control operation succeeds.
/// * `None` – If the device name is invalid or the operation fails.
///
/// # Safety
///
/// Although this wrapper is safe, be aware that `input_data` and `output_data` in the control block
/// are raw pointers. You must ensure those pointers are valid and the data they point to is
/// initialized, aligned, and sized correctly.
///
/// # Example
///
/// ```ignore
/// use rusty_threads::{device_control_block_t, device_control};
///
/// let mut input = [0u8; 512];
/// let mut output = [0u8; 512];
///
/// let ctrl = device_control_block_t {
///     command: rusty_threads::DISK_READ,
///     control1: 0, // e.g., sector
///     control2: 0, // e.g., platter
///     input_data: input.as_mut_ptr() as *mut _,
///     output_data: output.as_mut_ptr() as *mut _,
///     data_length: 512,
/// };
///
/// let result = device_control("disk0", ctrl);
/// assert!(result.is_some());
/// ```
pub fn device_control(device_name: &str, control_block: device_control_block_t) -> Option<u32> {
    let c_str = CString::new(device_name).expect("Device name contains null byte");
    let result = unsafe {
        c_device_control(c_str.as_ptr() as *mut _, control_block)
    };
    if result == u32::MAX {
        None
    } else {
        Some(result)
    }
}

/// Sets the debug level for THREADS console output.
///
/// This function controls the verbosity of output generated by the `console_output` function.
/// Setting the level to 0 disables debug output. Higher values enable progressively more verbose
/// debugging messages, depending on the kernel's internal debug implementation.
///
/// # Arguments
///
/// * `level` - An integer representing the desired debug verbosity level.
///             - `0` disables debug output.
///             - `1+` enables increasing levels of debug output (implementation-defined).
///
/// # Example
///
/// ```ignore
/// rusty_threads::set_debug_level(1); // Enable standard debug output
/// ```
pub fn set_debug_level(level: i32) {
    unsafe {
        crate::rusty_thread_bindings::c_set_debug_level(level);
    }
}


/// Prints formatted output to the console using the THREADS kernel's logging function.
///
/// This function wraps the variadic C `console_output` call, allowing Rust code
/// to use familiar `format!` syntax for building messages. It separates debug output
/// from normal output based on the `debug` flag.
///
/// # Arguments
///
/// * `debug` – `true` for debug output, `false` for standard output.
/// * `args` – A format string with parameters (like in `println!`).
///
/// # Example
///
/// ```ignore
/// rusty_threads::console_output(true, "Process {} started", pid);
/// rusty_threads::console_output(false, "System ready.");
/// ```
pub fn console_output(debug: bool, format: std::fmt::Arguments) {
    use std::fmt::Write;

    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", format);

    if let Ok(c_str) = CString::new(buffer) {
        unsafe {
            crate::rusty_thread_bindings::c_console_output(
                debug as bool,
                c_str.into_raw() as *mut i8,
            );
        }
    } 
}

/// Halts execution of the THREADS kernel.
///
/// This function will immediately terminate kernel execution. It should be used
/// only when the system is ready to shut down (e.g., during a clean exit or when
/// encountering a critical unrecoverable error).
///
/// # Arguments
///
/// * `code` – Exit status code.  
///     - `0` for a normal shutdown  
///     - Non-zero for errors or abnormal termination
///
/// # Panics
///
/// This function **does not return**. It transfers control to the kernel halt routine,
/// and any code after this call is unreachable.
///
/// # Safety
///
/// This function ultimately triggers a low-level halt — it will not unwind or clean
/// up Rust resources. Use only when you are sure shutdown is required.
///
/// # Example
///
/// ```ignore
/// // Clean shutdown
/// rusty_threads::stop(0);
///
/// // Shutdown due to error
/// rusty_threads::stop(1);
/// ```
pub fn stop(code: i32) -> ! {
    unsafe {
        crate::rusty_thread_bindings::c_stop(code);
    }
    unreachable!() // This line is never reached
}


/// Safely retrieves a mutable slice of the THREADS interrupt handler table.
///
/// This wraps the `c_get_interrupt_handlers()` function and exposes the result
/// as a safe mutable slice of interrupt handler function pointers. The slice
/// contains exactly [`THREADS_INTERRUPT_HANDLER_COUNT`] entries.
///
/// # Returns
///
/// A mutable slice of interrupt handler slots (each is `Option<unsafe extern "C" fn(...)>`).
///
/// # Safety
///
/// This function is safe because the pointer is immediately wrapped in a fixed-length slice,
/// and the size is guaranteed by the THREADS library.
///
/// # Example
///
/// ```ignore
/// unsafe extern "C" fn my_handler(id: [i8; 32], cmd: u8, status: u32) {
///     // Your logic here...
/// }
///
/// let handlers = rusty_threads::get_interrupt_handler_table();
/// handlers[rusty_threads::THREADS_TIMER_INTERRUPT as usize] = Some(my_handler);
/// ```
pub fn get_interrupt_handlers() -> &'static mut [interrupt_handler_t] {
    const COUNT: usize = THREADS_INTERRUPT_HANDLER_COUNT as usize;

    unsafe {
        let raw_ptr = c_get_interrupt_handlers();
        std::slice::from_raw_parts_mut(raw_ptr, COUNT)
    }
}


/// Retrieves the system call handler vector used by the THREADS kernel.
///
/// This vector contains pointers to functions responsible for handling specific system calls.
/// Each entry in the array corresponds to a syscall ID from 0 up to `THREADS_MAX_SYSCALLS - 1`.
///
/// To register a system call handler, assign a function to the appropriate index:
///
/// ```ignore
/// extern "C" fn my_syscall_handler(args: *mut system_call_arguments_t) {
///     // handle syscall
/// }
///
/// let syscall_vector = get_system_call_vector();
/// 
/// unsafe {
///     *syscall_vector.add(SYSCALL_ID) = Some(my_syscall_handler);
/// }
/// ```
///
/// # Safety
///
/// The caller must ensure that the function signature matches:
///
/// ```c 
/// typedef struct
/// {
///     uint32_t call_id;
///     uint32_t argDword
///     unit32_t argInt;
///     char *arg_string;
/// } system_call_arguments_t;
/// 
/// void handler(system_call_arguments_t* args);
/// 
/// ```
///
/// In Rust:
///
/// ```ignore
/// extern "C" fn handler(args: *mut system_call_arguments_t) { ... }
/// ```
///
/// # Returns
///
/// A mutable pointer to the system call handler vector.
pub fn get_system_call_vector() -> *mut system_call_handler_t {
    unsafe { c_get_system_call_vector() }
}