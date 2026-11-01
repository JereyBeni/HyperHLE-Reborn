/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Dependency-only stub for MetalKit.framework.
//!
//! This recognizes MetalKit's Mach-O install name without pretending that
//! MTKView, MTKTextureLoader, or MetalKit rendering are implemented.
//! The existing Metal.framework compatibility layer is separate.
//!
//! Keep this stub export-free until a guest binary demonstrates which
//! MetalKit symbols it actually imports. Add those APIs in Rust with
//! validated argument layouts and bounded memory access; do not fabricate
//! GPU work or return invalid guest pointers.

use crate::dyld::HostDylib;

pub const DYLIB: HostDylib = HostDylib {
    path: "/System/Library/Frameworks/MetalKit.framework/MetalKit",
    aliases: &[],
    class_exports: &[],
    constant_exports: &[],
    function_exports: &[],
};
