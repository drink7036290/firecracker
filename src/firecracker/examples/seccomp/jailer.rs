// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::env::args;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use vmm::seccomp::{apply_filter, deserialize_binary};

#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = args().collect();
    let exec_file = &args[1];
    let bpf_path = &args[2];

    let filter_file = File::open(bpf_path).unwrap();
    let map = deserialize_binary(&filter_file, None).unwrap();

    // Loads filters.
    apply_filter(map.get("main").unwrap()).unwrap();

    Command::new(exec_file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .exec();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    // On macOS or other platforms, this is a no-op
}