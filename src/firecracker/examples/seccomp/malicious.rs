// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

#[cfg(target_os = "linux")]
fn main() {
    unsafe {
        // In this example, the malicious component is outputting to standard input.
        libc::syscall(libc::SYS_write, libc::STDIN_FILENO, "Hello, world!\n", 14);
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    // On macOS or other platforms, this is a no-op
}