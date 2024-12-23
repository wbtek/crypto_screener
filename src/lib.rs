
// The MIT License (MIT)
//
// Copyright (c) 2024 Greg Slocum, WBTek,
// a division of WhiteBear Family, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Yew Application Entry Point
//!
//! This module serves as the main entry point for the Yew application, 
//! initializing the logger and rendering the main application model. 
//!
//! To start the application, use `run_app()` as defined below.

use yew::Renderer;
use wasm_bindgen::prelude::wasm_bindgen;

mod modules;
use modules::component::Model;

/// Entry point for the Yew application.
///
/// This function is called when the WebAssembly module is initialized
/// and starts the main Yew application. It also sets up a logger with
/// an `Info` level for debugging purposes.
///
/// # Examples
///
/// ```no_run
/// rust_doc_example::run_app();
/// ```
///
/// # Errors
///
/// If the logger is already initialized, a warning is logged
/// to indicate the duplicate initialization attempt, though 
/// the application proceeds normally.
///
/// # License
///
/// Distributed under the terms of the MIT license, which is included
/// in the root of the source tree and at the top of this file.
///
/// # Panics
///
/// Panics may occur if the `Renderer` fails to initialize due to
/// unforeseen configuration errors, though this is unlikely in most
/// deployment environments.
///
/// # See Also
/// * [`Renderer`](https://docs.rs/yew/latest/yew/struct.Renderer.html) - Renderer struct in Yew
/// * [`console_log`](https://docs.rs/console_log/latest/console_log/index.html) - Logging setup
#[wasm_bindgen(start)]
pub fn run_app() {
    if console_log::init_with_level(log::Level::Info).is_err() {
        log::warn!("Logger already initialized.");
    }
    log::info!("Starting the Yew application...");
    Renderer::<Model>::new().render();
}

