// Copyright 2024 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo::rustc-link-search=/usr/local/lib");
    println!("cargo::rustc-link-lib=seccomp");
}

#[cfg(not(target_os = "linux"))]
fn main() {
    // On macOS or other platforms, this is a no-op
}