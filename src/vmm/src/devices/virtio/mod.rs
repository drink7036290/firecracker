// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

//! Implements virtio devices, queues, and transport mechanisms.

#[cfg(target_os = "linux")]
use std::any::Any;

#[cfg(target_os = "linux")]
use self::queue::QueueError;
#[cfg(target_os = "linux")]
use crate::devices::virtio::net::TapError;

#[cfg(target_os = "linux")]
pub mod balloon;
pub mod block;
#[cfg(target_os = "linux")]
pub mod device;
#[cfg(target_os = "linux")]
pub mod gen;
#[cfg(target_os = "linux")]
mod iov_deque;
#[cfg(target_os = "linux")]
pub mod iovec;
#[cfg(target_os = "linux")]
pub mod mmio;
#[cfg(target_os = "linux")]
pub mod net;
#[cfg(target_os = "linux")]
pub mod persist;
#[cfg(target_os = "linux")]
pub mod queue;
#[cfg(target_os = "linux")]
pub mod rng;
#[cfg(target_os = "linux")]
pub mod test_utils;

#[cfg(target_os = "linux")]
pub mod vhost_user;
#[cfg(target_os = "linux")]
pub mod vhost_user_metrics;
#[cfg(target_os = "linux")]
pub mod vsock;

#[cfg(target_os = "linux")]
/// When the driver initializes the device, it lets the device know about the
/// completed stages using the Device Status Field.
///
/// These following consts are defined in the order in which the bits would
/// typically be set by the driver. INIT -> ACKNOWLEDGE -> DRIVER and so on.
///
/// This module is a 1:1 mapping for the Device Status Field in the virtio 1.0
/// specification, section 2.1.
mod device_status {
    pub const INIT: u32 = 0;
    pub const ACKNOWLEDGE: u32 = 1;
    pub const DRIVER: u32 = 2;
    pub const FAILED: u32 = 128;
    pub const FEATURES_OK: u32 = 8;
    pub const DRIVER_OK: u32 = 4;
    pub const DEVICE_NEEDS_RESET: u32 = 64;
}

#[cfg(target_os = "linux")]
/// Types taken from linux/virtio_ids.h.
/// Type 0 is not used by virtio. Use it as wildcard for non-virtio devices
/// Virtio net device ID.
pub const TYPE_NET: u32 = 1;
#[cfg(target_os = "linux")]
/// Virtio block device ID.
pub const TYPE_BLOCK: u32 = 2;
#[cfg(target_os = "linux")]
/// Virtio rng device ID.
pub const TYPE_RNG: u32 = 4;
#[cfg(target_os = "linux")]
/// Virtio balloon device ID.
pub const TYPE_BALLOON: u32 = 5;

#[cfg(target_os = "linux")]
/// Offset from the base MMIO address of a virtio device used by the guest to notify the device of
/// queue events.
pub const NOTIFY_REG_OFFSET: u32 = 0x50;

#[cfg(target_os = "linux")]
/// Errors triggered when activating a VirtioDevice.
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum ActivateError {
    /// Wrong number of queue for virtio device: expected {expected}, got {got}
    QueueMismatch { expected: usize, got: usize },
    /// Failed to write to activate eventfd
    EventFd,
    /// Vhost user: {0}
    VhostUser(vhost_user::VhostUserError),
    /// Setting tap interface offload flags failed: {0}
    TapSetOffload(TapError),
    /// Error setting pointers in the queue: (0)
    QueueMemoryError(QueueError),
}

#[cfg(target_os = "linux")]
/// Trait that helps in upcasting an object to Any
pub trait AsAny {
    /// Return the immutable any encapsulated object.
    fn as_any(&self) -> &dyn Any;

    /// Return the mutable encapsulated any object.
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

#[cfg(target_os = "linux")]
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
