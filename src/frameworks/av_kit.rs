/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Dependency-only stub for AVKit.framework.
//!
//! This recognizes the framework's Mach-O install name so apps that merely
//! link AVKit do not fail during dependency loading. No AVKit classes,
//! selectors, constants, or playback behavior are implemented here.
//! Add only APIs confirmed to be used by a guest app, with safe no-op or
//! nil-returning behavior where that matches the API contract.

use crate::dyld::HostDylib;

pub const DYLIB: HostDylib = HostDylib {
    path: "/System/Library/Frameworks/AVKit.framework/AVKit",
    aliases: &[],
    class_exports: &[],
    constant_exports: &[],
    function_exports: &[],
};
