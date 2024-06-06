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

use yew::prelude::*;
use crate::modules::json::CryptoData;
use wasm_bindgen::prelude::*;

struct Model {
    data: Vec<CryptoData>,
    error_message: Option<String>,
}

pub enum Msg {
    FetchData,
    SetData(Result<Vec<CryptoData>, reqwest::Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchData);
        Self { data: Vec::new(), error_message: None }
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
                        self.data = data;
                        self.error_message = None;
                    },
                    Err(err) => {
                        self.error_message = Some(format!("Failed to fetch data: {:?}", err));
                    },
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                        <tr>
                            <th style="text-align: left;">{ "Symbol" }</th>
                            <th style="text-align: left;">{ "Name" }</th>
                            <th style="text-align: right;">{ "Price (USD)" }</th>
                            <th style="text-align: right;">{ "1h %" }</th>
                            <th style="text-align: right;">{ "24h %" }</th>
                            <th style="text-align: right;">{ "7d %" }</th>
                            <th style="text-align: right;">{ "Volume ($)" }</th>
                        </tr>
                        <tr>
                            <th colspan="7" style="text-align: left;">{ underscore_line.clone() }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for self.data.iter().map(|item| {
                            html! {
                                <tr>
                                    <td style="text-align: left;">{ item.symbol.clone().unwrap_or_default() }</td>
                                    <td style="text-align: left;">{ item.truncated_name() }</td>
                                    <td style="text-align: right;">{ item.formatted_price() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_1h() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_24h() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_7d() }</td>
                                    <td style="text-align: right;">{ item.formatted_volume() }</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Model>::new().render();
}

