// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

extern "C" {
    fn caml_startup(argv: *const *const i8);
    fn caml_shutdown();
}

/// OCaml runtime handle.
pub struct OCamlRuntime {}

impl OCamlRuntime {
    /// Initializes the OCaml runtime and returns a handle, that once dropped
    /// will perform the necessary cleanup.
    pub fn init() -> Self {
        OCamlRuntime::init_persistent();
        OCamlRuntime {}
    }

    /// Initializes the OCaml runtime.
    pub fn init_persistent() {
        let arg0 = "ocaml".as_ptr() as *const i8;
        let c_args = vec![arg0, std::ptr::null()];
        unsafe { caml_startup(c_args.as_ptr()) }
    }

    /// Performs the necessary cleanup and shuts down the OCaml runtime.
    pub fn shutdown_persistent() {
        unsafe { caml_shutdown() }
    }
}

impl Drop for OCamlRuntime {
    fn drop(&mut self) {
        unsafe { caml_shutdown() }
    }
}
