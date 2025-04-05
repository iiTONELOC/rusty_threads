// src/constants.rs

/// Minimum allowed process stack size.
pub const THREADS_MIN_STACK_SIZE: u32 = 8192;

/// Maximum process name length.
pub const THREADS_MAX_NAME: u32 = 128;

/// Maximum number of syscalls supported.
pub const THREADS_MAX_SYSCALLS: u32 = 32;

/// Maximum number of disks supported.
pub const THREADS_MAX_DISKS: u32 = 4;

/// Maximum number of terminals supported.
pub const THREADS_MAX_TERMINALS: u32 = 4;

/// ID used for the clock device.
pub const THREADS_CLOCK_DEVICE_ID: u32 = 0;

/// Maximum number of devices supported.
pub const THREADS_MAX_DEVICES: u32 = 8;

/// Maximum I/O buffer size.
pub const THREADS_MAX_IO_BUFFER_SIZE: u32 = 1024;

/// Maximum device name length.
pub const THREADS_MAX_DEVICE_NAME: u32 = 32;

/// Maximum number of processes allowed.
pub const MAX_PROCESSES: u32 = 50;

/// Timer interrupt ID.
pub const THREADS_TIMER_INTERRUPT: u32 = 0;

/// I/O interrupt ID.
pub const THREADS_IO_INTERRUPT: u32 = 1;

/// Exception interrupt ID.
pub const THREADS_EXCEPTION_INTERRUPT: u32 = 2;

/// System call interrupt ID.
pub const THREADS_SYS_CALL_INTERRUPT: u32 = 3;

/// Number of entries in the interrupt vector.
pub const THREADS_INTERRUPT_HANDLER_COUNT: u32 = 4;

/// Bitmask: Interrupts enabled.
pub const PSR_INTERRUPTS: u32 = 1;

/// Bitmask: Kernel mode active.
pub const PSR_KERNEL_MODE: u32 = 2;

/// Bitmask: IRQ mode active.
pub const PSR_IRQ_MODE: u32 = 4;

/// Disk command: Retrieve disk info.
pub const DISK_INFO: u8 = 0x01;

/// Disk command: Read from disk.
pub const DISK_READ: u8 = 0x04;

/// Disk command: Write to disk.
pub const DISK_WRITE: u8 = 0x08;

/// Disk command: Seek to location.
pub const DISK_SEEK: u8 = 0x10;

/// Terminal command: Read one character.
pub const TERMINAL_READ_CHAR: u8 = 0x20;

/// Terminal command: Write one character.
pub const TERMINAL_WRITE_CHAR: u8 = 0x40;

/// Disk sector size in bytes.
pub const THREADS_DISK_SECTOR_SIZE: u32 = 512;

/// Number of sectors per disk track.
pub const THREADS_DISK_SECTOR_COUNT: u32 = 16;

/// Maximum number of disk platters.
pub const THREADS_DISK_MAX_PLATTERS: u32 = 3;
