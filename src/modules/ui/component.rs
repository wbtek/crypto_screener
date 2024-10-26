
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

use yew::prelude::*;
use std::collections::HashSet;
use crate::modules::json::CryptoData;
use crate::modules::sort::sort_data;
use crate::modules::ui::message::Msg;
use crate::modules::ui::headview::view_header;
use crate::modules::ui::rowview::view_rows;
use crate::modules::ui::utils::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static COMPONENT_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Model {
    pub data: Vec<CryptoData>,
    pub error_message: Option<String>,
    pub show_about: bool,  // Track visibility of "About" modal
    pub sort_by: Option<String>,
    pub sort_asc: bool,
    pub selected_cells: HashSet<(String, String)>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let count = COMPONENT_INIT_COUNT.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            log::info!("Model component created: {}", count);
            ctx.link().send_message(Msg::FetchData); // Ensure this only runs once
        } else {
            log::warn!("Model component created multiple times: {}", count);
        }

        Self { 
            data: Vec::new(), 
            error_message: None, 
            show_about: false,
            sort_by: Some("volume24".to_string()), // Sort this out of the gate
            sort_asc: false, // Descending
            selected_cells: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                log::info!("Fetching data"); // Log when fetching data
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let data = crate::modules::http::fetch_data().await;
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
                    self.sort_asc = column.eq("symbol") || column.eq("name"); // Sort the rest descending
                }
                sort_data(&mut self.data, &self.sort_by, self.sort_asc);
                true
            }
            Msg::ToggleCellSelection(id, column) => {
                toggle_cell_selection(&mut self.selected_cells, id, column);
                true
            }
            Msg::ToggleAbout => {
                self.show_about = !self.show_about; // Toggle visibility
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let underscore_line = "_".repeat(130);
        html! {
            <div>
                <br />
                <div style="display: flex; align-items: center;">
                    <h1 style="margin: 0; margin-right: 30px;">{ "WBTek Crypto Screener" }</h1>
                    <button onclick={ctx.link().callback(|_| Msg::ToggleAbout)}>{ "About" }</button>
                </div>
                <br />

                { if self.show_about {
                    html! {
                        <div class="modal">
                            <div class="modal-content">
                                <span class="close" onclick={ctx.link().callback(|_| Msg::ToggleAbout)}>{ "×" }</span>
                                <h2> { "About WBTek Crypto Screener" } </h2>
                                <p>
                                    { "Click on header buttons to sort data, and click on individual cells to highlight them." } <br />
                                    <br />
                                    { "Built with Rust and Yew, compiled to WebAssembly (WASM)." } <br />
                                    <br />
                                    <a href="https://github.com/wbtek/crypto_screener"
                                        target="_blank">{ "Source: https://github.com/wbtek/crypto_screener" }
                                    </a>
                                    <br />
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
                                </p>
                            </div>
                        </div>
                    }
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

