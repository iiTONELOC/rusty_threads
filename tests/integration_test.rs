use rusty_threads::*;

#[allow(unused_comparisons)]
#[test]
fn test_constants_accessible() {
     // Check that the constants are defined and accessible
     assert!(THREADS_MAX_SYSCALLS > 0);
     assert!(THREADS_MAX_DEVICES > 0);
     assert!(THREADS_MAX_IO_BUFFER_SIZE > 0);
     assert!(THREADS_MAX_NAME > 0);
     assert!(THREADS_MIN_STACK_SIZE > 0);
     assert!(THREADS_MAX_DISKS > 0);
     assert!(THREADS_MAX_TERMINALS > 0);
     assert!(THREADS_CLOCK_DEVICE_ID >= 0); // should be 0
     assert!(MAX_PROCESSES > 0);
     assert!(THREADS_TIMER_INTERRUPT >= 0); // should be 0
     assert!(THREADS_IO_INTERRUPT > 0);
     assert!(THREADS_EXCEPTION_INTERRUPT > 0);
     assert!(THREADS_SYS_CALL_INTERRUPT > 0);
     assert!(THREADS_INTERRUPT_HANDLER_COUNT > 0);
     assert!(PSR_INTERRUPTS > 0);
     assert!(PSR_KERNEL_MODE > 0);
     assert!(PSR_IRQ_MODE > 0);
     assert!(DISK_INFO > 0);
     assert!(DISK_READ > 0);
     assert!(DISK_WRITE > 0);
     assert!(DISK_SEEK > 0);
     assert!(TERMINAL_READ_CHAR > 0);
     assert!(TERMINAL_WRITE_CHAR > 0);
     assert!(THREADS_DISK_SECTOR_COUNT > 0);
     assert!(THREADS_DISK_SECTOR_SIZE > 0);
     assert!(THREADS_DISK_MAX_PLATTERS > 0);
}

#[test]
fn test_constants_values() {
    // Check that the constants have expected values
    assert_eq!(THREADS_MAX_SYSCALLS, 32);
    assert_eq!(THREADS_MAX_DEVICES, 8);
    assert_eq!(THREADS_MAX_IO_BUFFER_SIZE, 1024);
    assert_eq!(THREADS_MAX_NAME, 128);
    assert_eq!(THREADS_MIN_STACK_SIZE, 8192);
    assert_eq!(THREADS_MAX_DISKS, 4);
    assert_eq!(THREADS_MAX_TERMINALS, 4);
    assert_eq!(THREADS_CLOCK_DEVICE_ID, 0); // should be 0
    assert_eq!(MAX_PROCESSES, 50);
    assert_eq!(THREADS_TIMER_INTERRUPT, 0); // should be 0
    assert_eq!(THREADS_IO_INTERRUPT, 1);
    assert_eq!(THREADS_EXCEPTION_INTERRUPT, 2);
    assert_eq!(THREADS_SYS_CALL_INTERRUPT, 3);
    assert_eq!(THREADS_INTERRUPT_HANDLER_COUNT, 4);
    assert_eq!(PSR_INTERRUPTS, 1);
    assert_eq!(PSR_KERNEL_MODE, 2);
    assert_eq!(PSR_IRQ_MODE, 4);
    assert_eq!(DISK_INFO, 0x01);
    assert_eq!(DISK_READ, 0x04);
    assert_eq!(DISK_WRITE, 0x08);
    assert_eq!(DISK_SEEK, 0x10);
    assert_eq!(TERMINAL_READ_CHAR, 0x20);
    assert_eq!(TERMINAL_WRITE_CHAR, 0x40);
    assert_eq!(THREADS_DISK_SECTOR_COUNT, 16);
    assert_eq!(THREADS_DISK_SECTOR_SIZE, 512);
    assert_eq!(THREADS_DISK_MAX_PLATTERS, 3);
}

#[test]
fn test_public_wrapper_accessability() {
    // Check that the public wrapper functions are accessible
    assert!(rusty_wrapper::console_output as usize != 0);
    assert!(rusty_wrapper::context_initialize as usize != 0);
    assert!(rusty_wrapper::context_stop as usize != 0);
    assert!(rusty_wrapper::context_switch as usize != 0);
    assert!(rusty_wrapper::device_control as usize != 0);
    assert!(rusty_wrapper::device_handle as usize != 0);
    assert!(rusty_wrapper::device_initialize as usize != 0);
    assert!(rusty_wrapper::get_interrupt_handlers as usize != 0);
    assert!(rusty_wrapper::get_psr as usize != 0);
    assert!(rusty_wrapper::set_debug_level as usize != 0);
    assert!(rusty_wrapper::set_psr as usize != 0);
    assert!(rusty_wrapper::stop as usize != 0);
    assert!(rusty_wrapper::system_clock as usize != 0);
}


