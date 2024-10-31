
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

//! # Main Application Model for WBTek Crypto Screener
//!
//! This module defines the main application model (`Model`) and handles core application functionality
//! such as fetching data, updating the UI, sorting, and managing selected cells. It coordinates with 
//! various submodules and components to render the "Crypto Screener" interface in a Yew application.
//!
//! ## Main Components
//!
//! - `Model`: The main application state, managing data, sort state, and UI visibility.
//! - `Msg`: Message types used to trigger updates to the application state and user interactions.
//! - `COMPONENT_INIT_COUNT`: Tracks component initialization to ensure certain actions (e.g., initial data fetch) 
//!   run only once.

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use yew::prelude::{Component, Context, Html, html};
use super::about::about_view;
use super::cryptodata::CryptoData;
use super::fetch::fetch_data;
use super::headview::view_header;
use super::sort::sort_data;
use super::message::Msg;
use super::rowview::view_rows;
use super::utils::toggle_cell_selection;

/// Tracks the number of `Model` instances created to ensure initial setup
/// (such as data fetch) only happens once. This helps prevent redundant 
/// initialization if the component is accidentally created multiple times.
static COMPONENT_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Represents the main application model, managing the application's core state.
///
/// `Model` includes fields for application data, error messages, modal visibility, 
/// sorting options, and cell selections. It implements the Yew `Component` trait, 
/// handling updates and rendering based on incoming messages (`Msg`).
pub struct Model {
    /// The list of cryptocurrency data currently loaded.
    pub data: Vec<CryptoData>,
    
    /// Holds error messages from failed data fetch attempts, displayed in the UI.
    pub error_message: Option<String>,
    
    /// Tracks visibility of the "About" modal.
    pub show_about: bool,
    
    /// Specifies the current column used for sorting.
    pub sort_by: Option<String>,
    
    /// Indicates whether sorting is in ascending order.
    pub sort_asc: bool,
    
    /// HashSet of selected cells, represented by unique `(id, column)` pairs.
    pub selected_cells: HashSet<(String, String)>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    /// Creates the `Model` component, initializing data and triggering an initial data fetch
    /// if this is the first instance. Logs a warning if additional instances are created.
    fn create(ctx: &Context<Self>) -> Self {
        let count = COMPONENT_INIT_COUNT.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            log::info!("Model component created: {}", count);
            ctx.link().send_message(Msg::FetchData); // Initial data fetch
        } else {
            log::warn!("Model component created multiple times: {}", count);
        }

        Self { 
            data: Vec::new(), 
            error_message: None, 
            show_about: false,
            sort_by: Some("volume24".to_string()), // Initial sort by "volume24"
            sort_asc: false, // Default to descending
            selected_cells: HashSet::new(),
        }
    }

    /// Updates the `Model` state in response to various messages (`Msg`).
    ///
    /// The `update` function handles different actions based on the incoming message,
    /// such as fetching data, setting data, toggling the sort order, and managing 
    /// the selection state of cells.
    ///
    /// # Parameters
    ///
    /// - `ctx`: Reference to the component’s context.
    /// - `msg`: The message triggering the update.
    ///
    /// # Returns
    ///
    /// Returns `true` if the UI needs to be re-rendered after processing the message.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                log::info!("Fetching data");
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let data = fetch_data().await;
                    link.send_message(Msg::SetData(data));
                });
                false
            }
            Msg::SetData(result) => {
                match result {
                    Ok(data) => {
                        self.data = data;
                        self.error_message = None;
                        sort_data(&mut self.data, &self.sort_by, self.sort_asc);
                    },
                    Err(err) => {
                        self.error_message = Some(format!("Failed to fetch data: {:?}", err));
                    },
                }
                true
            }
            Msg::SortBy(column) => {
                if self.sort_by.as_ref() == Some(&column) {
                    self.sort_asc = !self.sort_asc;
                } else {
                    self.sort_by = Some(column.clone());
                    self.sort_asc = column.eq("symbol") || column.eq("name"); // Sort alphabetically ascending
                }
                sort_data(&mut self.data, &self.sort_by, self.sort_asc);
                true
            }
            Msg::ToggleCellSelection(id, column) => {
                toggle_cell_selection(&mut self.selected_cells, id, column);
                true
            }
            Msg::ToggleAbout => {
                self.show_about = !self.show_about; // Toggle "About" modal visibility
                true
            }
        }
    }

    /// Renders the main view of the application.
    ///
    /// This function creates the application layout, including the title, the
    /// "About" button, the error message (if any), and the sortable data table.
    /// It also conditionally renders the "About" modal if `show_about` is `true`.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let underscore_line = "_".repeat(130); // For visual structure

        html! {
            <div>
                <br />
                <div style="display: flex; align-items: center;">
                    <h1 style="margin: 0; margin-right: 30px;">{ "WBTek Crypto Screener" }</h1>
                    <button onclick={ctx.link().callback(|_| Msg::ToggleAbout)}>{ "About" }</button>
                </div>
                <br />

                { if self.show_about {
                    about_view(ctx)
                } else {
                    html! {}
                }}
     
                { if let Some(error) = &self.error_message {
                    html! { <p style="color: red;">{ error }</p> }
                } else {
                    html! {}
                }}
                <table>
                    <thead>
                        { view_header(ctx, &underscore_line, self) }
                    </thead>
                    <tbody>
                        { view_rows(ctx, self) }
                    </tbody>
                </table>
            </div>
        }
    }
}
