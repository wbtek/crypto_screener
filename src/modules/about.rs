
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

//! # About Modal Module
//!
//! This module defines the `about_view` function, which renders the "About" modal
//! for the WBTek Crypto Screener application. The modal provides information about
//! the application's usage, source code, and license details.
//!
//! ## Purpose
//!
//! - To display usage instructions, so users can interact effectively with
//!   the table and data presented in the app.
//! - To provide links to source code, documentation, and the company homepage.
//! - To display licensing information, ensuring users understand the terms of
//!   use for this software.
//!
//! ## Structure
//!
//! This module relies on components from `super::component` and `super::message`
//! to interact with the main application state and emit messages for user actions.
//! 
//! ### Key Function
//! - `about_view`: A function that returns an HTML structure to render the modal
//!   and closes the modal upon clicking the "×" button.
//!
//! This module is typically used in the context of Yew applications and assumes
//! an HTML/CSS structure compatible with the Yew framework.

use yew::{html, Html, Context};
use super::component::Model;
use super::message::Msg;

/// Renders the "About" modal for the WBTek Crypto Screener application.
///
/// This modal provides an overview of the application, including usage instructions,
/// project links, and the full MIT License. It also includes interactive functionality 
/// for closing the modal through a close button.
///
/// # Parameters
///
/// - `ctx`: A reference to the Yew [`Context`] associated with the main application [`Model`].
///    This provides access to the component's context, allowing interaction through messages,
///    such as closing the modal by sending [`Msg::ToggleAbout`].
///
/// # Returns
///
/// Returns an [`Html`] object, representing the full HTML structure of the "About" modal.
///
/// # Structure
///
/// The modal includes the following sections:
/// 
/// - A **close button** (`×`), which uses `ctx.link().callback()` to emit a message 
///   that closes the modal when clicked.
/// - **Header**: Displays the title "About WBTek Crypto Screener."
/// - **Application Instructions**: Details how users can interact with the table headers to 
///   sort data and click individual cells to highlight them.
/// - **Project Links**: Provides links to the source code and project documentation,
///   along with the WBTek homepage, each opening in a new tab.
/// - **MIT License**: Displays the full text of the MIT License to clarify usage rights and 
///   permissions associated with the software.
///
/// # Example
///
/// ```rust
/// // Render the "About" modal in the application using the context `ctx`.
/// let about_html = about_view(ctx);
/// ```
///
/// # See Also
/// - [`Context`]: Yew documentation on component context for managing component state and interactions.
/// - [`Html`]: Yew’s HTML type for representing HTML in Rust.
/// - [`Msg::ToggleAbout`]: Message variant used to toggle the "About" modal on and off.
pub fn about_view(ctx: &Context<Model>) -> Html {
    html! {
        <div class="modal">
            <div class="modal-content">
                <span class="close" onclick={ctx.link().callback(|_| Msg::ToggleAbout)}>{ "\u{00D7}" }</span> // multiply char
                <h2> { "About WBTek Crypto Screener" } </h2>
                <p>
                    { "Click on header buttons to sort data, and click on individual cells to highlight them." } <br />
                    <br />
                    { "Built with Rust and Yew, compiled to WebAssembly (WASM)." } <br />
                    <br />
                    { "Source and documentation: " }
                        <a href="https://wbtek.github.io"
                            target="_blank">{ "https://wbtek.github.io" }
                    </a>
                    <br />
                    { "WBTek's Homepage: " }
                        <a href="https://wbtek.slocum.net"
                            target="_blank">{ "https://wbtek.slocum.net" }
                    </a>

                    <hr />
                    <br />
                    { "The MIT License (MIT)" } <br />
                    <br />
                    { "Copyright (c) 2024 Greg Slocum, WBTek," } <br />
                    { "a division of WhiteBear Family, Inc." } <br />
                    <br />
                    { "Permission is hereby granted, free of charge, to any person obtaining a copy
                    of this software and associated documentation files (the \"Software\"), to deal
                    in the Software without restriction, including without limitation the rights
                    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                    copies of the Software, and to permit persons to whom the Software is
                    furnished to do so, subject to the following conditions:" } <br />
                    <br />
                    { "The above copyright notice and this permission notice shall be included in all
                    copies or substantial portions of the Software." } <br />
                    <br />
                    { "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                    SOFTWARE." } <br />
                    <br />
                    <hr />
                    <br />
                </p>
                // Identification marker
                <p style="font-size: 0.7em;">{ "(0078)" }</p>
            </div>
        </div>
    }
}
