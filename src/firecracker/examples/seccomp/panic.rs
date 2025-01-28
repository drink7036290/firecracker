// Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

#[cfg(target_os = "linux")]
use std::env::args;
#[cfg(target_os = "linux")]
use std::fs::File;

#[cfg(target_os = "linux")]
use vmm::seccomp::{apply_filter, deserialize_binary};

#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = args().collect();
    let bpf_path = &args[1];
    let filter_thread = &args[2];

    let filter_file = File::open(bpf_path).unwrap();
    let map = deserialize_binary(&filter_file, None).unwrap();
    apply_filter(map.get(filter_thread).unwrap()).unwrap();
    panic!("Expected panic.");
}

#[cfg(not(target_os = "linux"))]
fn main() {
    // On macOS or other platforms, this is a no-op
}