# Rusty Threads

⚙️ `rusty_threads` is a Rust wrapper around the THREADS C-based kernel simulator.

## Table of Contents

- [Rusty Threads](#rusty-threads)
  - [Table of Contents](#table-of-contents)
  - [Screenshot](#screenshot)
  - [Description](#description)
  - [Overview](#overview)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Contributors](#contributors)
  - [References](#references)

## Screenshot

![Threads - University of Arizona](./assets/threads-ua.png)

## Description

> THREADS provides a teaching platform for kernel-level concurrency and scheduling by simulating low-level CPU behavior, allowing students to develop, test, and visualize thread execution in a simplified kernel environment. This simulator helps bridge the gap between abstract OS concepts and real-world systems programming.  
> (University of Arizona, n.d.)

## Overview

For now, Rusty Threads provides a Rust wrapper around the THREADS.dll/lib, which is already implemented in C.

It makes use of [Rust's FFI](https://doc.rust-lang.org/nomicon/ffi.html#foreign-function-interface) to call C code directly from Rust.

A Rust crate [`bindgen`](https://rust-lang.github.io/rust-bindgen/requirements.html), was used to generate the necessary FFI bindings, automatically, based on the [`THREADSLib.h`](./include/THREADSLib.h) header file.

The approach with `Rusty Threads` is to wrap all `unsafe` calls to c code in a Rust wrapper so that the `unsafe` methods are never accessed directly and raw memory is never played with, or touched.

For now, any and all 'tests' simply assert the bindings are valid. No further assertions have been made.

## Getting Started

>Note: Since THREADS uses the Win API, it is assumed (wrongly) that all commands are being executed on Windows via PowerShell.

### Prerequisites

- Windows 10, 11
- University of Arizona Student/Faculty email address (Required for access to THREADS binaries)
- Rust 2024
  ([RFC #3501](https://rust-lang.github.io/rfcs/3501-edition-2024.html)) Release - 1.85.0
- Clang, necessary for bindgen.

1. Reach out to [Professor Duren](mailto:mduren@arizona.edu) with a University of Arizona email address to obtain a copy of THREADS if needed.

2. If not installed, [Rust can be installed via rustup](https://www.rust-lang.org/tools/install).  
  2.1. If new to Rust , check out their [Getting Started Guide](https://www.rust-lang.org/learn/get-started).

3. Install Clang:

    ```powershell
    # via winget - Recommended
    winget install LLVM.LLVM
  
    # via pacman
    pacman -S  mingw64/mingw-w64-x86_64-clang
    ```

4. Add `LIBCLANG_PATH` to [environment variables](https://www.alphr.com/environment-variables-windows-10/) and point it to the `bin\` in the directory where Clang was installed. If using the default installation location then the path should be: `C:\Program Files\LLVM\bin`. Or you can run the following in PowerShell, be sure to adjust the path as needed:

   ```powershell
    # add to current session
    $env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
  
    # set permanently as a User scoped env
    [Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "User")

    # verify 
    Get-ItemProperty -Path "HKCU:\Environment" | Select-Object LIBCLANG_PATH
   ```

### Installation

1. Clone the repository:

    ```powershell
    git clone git@github.com:iiTONELOC/rusty_threads.git
    ```

2. Add `THREADS.dll` and `THREADS.lib` to the `/lib` folder, as these files are excluded from version control.

3. Build the library using the `cargo build` command:

    ```powershell
    cd <path_to_rusty_threads>; cargo build
    ```

4. Verify everything was built and linked with the `cargo test` command:

```powershell
cargo test

# example output
#-----------------

running 1 test
test rusty_thread_bindings::tests::test_bindings_linkage ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (target\debug\deps\integration_test-eb6a4b0881a4586e.exe)

running 3 tests
test test_public_wrapper_accessability ... ok
test test_constants_values ... ok
test test_constants_accessible ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Contributors

| Name | College | Program | Contact |
| --- | --- | --- | --- |
| Anthony Tropeano | [**College of Applied Science and Technology**](https://azcast.arizona.edu/) | [*Cyber Operations/Cyber Engineering*](https://azcast.arizona.edu/academics/cyber-operations/cyber-engineering) | [**GitHub**](https://github.com/iiTONELOC) <br> [**Email**](mailto:atropeano@arizona.edu) |

## References

University of Arizona, College of Science and Technology. (n.d.). *THREADS (Threaded Heuristic Research, Educational, And Development Simulator): User manual* (Version 1.0) [Unpublished internal document]. University of Arizona.
