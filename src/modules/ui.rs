// MIT License
//
// Copyright (c) 2024 - WBTek: Greg Slocum
// Division of WhiteBear Family, Inc.
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

use std::collections::HashSet;
use yew::prelude::*;
use crate::modules::json::CryptoData;
use crate::modules::sort::sort_data;
use crate::modules::but::{HeaderButton, SortOrder};
use wasm_bindgen::prelude::*;

struct Model {
    data: Vec<CryptoData>,
    error_message: Option<String>,
    sort_by: Option<String>,
    sort_asc: bool,
    selected_cells: HashSet<(String, String)>, // Use unique id and column name
}

pub enum Msg {
    FetchData,
    SetData(Result<Vec<CryptoData>, reqwest::Error>),
    SortBy(String),
    ToggleCellSelection(String, String), // Use unique id
}

impl Model {
    fn sort_order(&self, column: &str) -> SortOrder {
        if let Some(ref sort_by) = self.sort_by {
            if sort_by == column {
                if self.sort_asc {
                    return SortOrder::Ascending;
                } else {
                    return SortOrder::Descending;
                }
            }
        }
        SortOrder::None
    }

    fn handle_sort(&mut self, column: String) {
        if self.sort_by.as_ref() == Some(&column) {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort_by = Some(column);
            self.sort_asc = true;
        }
        sort_data(&mut self.data, &self.sort_by, self.sort_asc);
    }

    fn toggle_cell_selection(&mut self, id: String, column: String) {
        // log::info!("Toggling selection for cell: id={}, column={}", id, column);
        let cell = (id.clone(), column.clone());
        if self.selected_cells.contains(&cell) {
            self.selected_cells.remove(&cell);
            // log::info!("Cell deselected: {:?}", cell);
        } else {
            self.selected_cells.insert(cell.clone());
            // log::info!("Cell selected: {:?}", cell);
        }
        // log::info!("Current selected cells: {:?}", self.selected_cells);
    }

    fn cell_style(&self, id: &str, column: &str) -> String {
        if self.selected_cells.contains(&(id.to_string(), column.to_string())) {
            "background-color: orange;".to_string()
        } else {
            "".to_string()
        }
    }

    fn view_header(&self, ctx: &Context<Self>, underscore_line: &str) -> Html {
        let link = ctx.link();
        html! {
            <>
                <tr>
                    <HeaderButton
                        label={"Symbol".to_string()}
                        sort_order={self.sort_order("symbol")}
                        is_numeric={false}
                        onclick={link.callback(|_| Msg::SortBy("symbol".to_string()))}
                    />
                    <HeaderButton
                        label={"Name".to_string()}
                        sort_order={self.sort_order("name")}
                        is_numeric={false}
                        onclick={link.callback(|_| Msg::SortBy("name".to_string()))}
                    />
                    <HeaderButton
                        label={"Price (USD)".to_string()}
                        sort_order={self.sort_order("price_usd")}
                        is_numeric={true}
                        onclick={link.callback(|_| Msg::SortBy("price_usd".to_string()))}
                    />
                    <HeaderButton
                        label={"1h %".to_string()}
                        sort_order={self.sort_order("percent_change_1h")}
                        is_numeric={true}
                        onclick={link.callback(|_| Msg::SortBy("percent_change_1h".to_string()))}
                    />
                    <HeaderButton
                        label={"24h %".to_string()}
                        sort_order={self.sort_order("percent_change_24h")}
                        is_numeric={true}
                        onclick={link.callback(|_| Msg::SortBy("percent_change_24h".to_string()))}
                    />
                    <HeaderButton
                        label={"7d %".to_string()}
                        sort_order={self.sort_order("percent_change_7d")}
                        is_numeric={true}
                        onclick={link.callback(|_| Msg::SortBy("percent_change_7d".to_string()))}
                    />
                    <HeaderButton
                        label={"Volume ($)".to_string()}
                        sort_order={self.sort_order("volume24")}
                        is_numeric={true}
                        onclick={link.callback(|_| Msg::SortBy("volume24".to_string()))}
                    />
                </tr>
                <tr>
                    <th colspan="7" style="text-align: left;">{ underscore_line }</th>
                </tr>
            </>
        }
    }

    fn view_rows(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            { for self.data.iter().enumerate().map(|(_row_index, item)| {
                let id = item.symbol.clone().unwrap_or_default();
                html! {
                    <tr>
                        <td
                            style={self.cell_style(&id, "symbol")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "symbol".to_string())
                            })}
                        >{ item.symbol.clone().unwrap_or_default() }</td>
                        <td
                            style={self.cell_style(&id, "name")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "name".to_string())
                            })}
                        >{ item.truncated_name() }</td>
                        <td
                            style={self.cell_style(&id, "price_usd")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "price_usd".to_string())
                            })}
                        >{ item.formatted_price() }</td>
                        <td
                            style={self.cell_style(&id, "percent_change_1h")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_1h".to_string())
                            })}
                        >{ item.formatted_percent_change_1h() }</td>
                        <td
                            style={self.cell_style(&id, "percent_change_24h")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_24h".to_string())
                            })}
                        >{ item.formatted_percent_change_24h() }</td>
                        <td
                            style={self.cell_style(&id, "percent_change_7d")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_7d".to_string())
                            })}
                        >{ item.formatted_percent_change_7d() }</td>
                        <td
                            style={self.cell_style(&id, "volume24")}
                            onclick={link.callback({
                                let id = id.clone();
                                move |_| Msg::ToggleCellSelection(id.clone(), "volume24".to_string())
                            })}
                        >{ item.formatted_volume() }</td>
                    </tr>
                }
            }) }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchData);
        Self { 
            data: Vec::new(), 
            error_message: None, 
            sort_by: None, 
            sort_asc: true, 
            selected_cells: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
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
                        // log::info!("SetData received: {:?}", data);
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
                self.handle_sort(column);
                true
            }
            Msg::ToggleCellSelection(id, column) => {
                self.toggle_cell_selection(id, column);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let underscore_line = "_".repeat(130);
        html! {
            <div>
                <h1>{ "Crypto Screener" }</h1>
                { if let Some(error) = &self.error_message {
                    html! { <p style="color: red;">{ error }</p> }
                } else {
                    html! {}
                }}
                <table>
                    <thead>
                        { self.view_header(ctx, &underscore_line) }
                    </thead>
                    <tbody>
                        { self.view_rows(ctx) }
                    </tbody>
                </table>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    yew::Renderer::<Model>::new().render();
}

