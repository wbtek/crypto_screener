
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
use crate::modules::ui::component::Model;
use crate::modules::ui::message::Msg;
use crate::modules::ui::utils::cell_style;

pub fn view_rows(ctx: &Context<Model>, model: &Model) -> Html {
    let link = ctx.link();
    html! {
        { for model.data.iter().map(|item| {
            let id = item.symbol.clone().unwrap_or_default();
            html! {
                <tr>
                    <td
                        style={cell_style(&id, "symbol", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "symbol".to_string())
                        })}
                    >{ item.symbol.clone().unwrap_or_default() }</td>
                    <td
                        style={cell_style(&id, "name", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "name".to_string())
                        })}
                    >{ item.truncated_name() }</td>
                    <td
                        style={cell_style(&id, "price_usd", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "price_usd".to_string())
                        })}
                    >{ item.formatted_price() }</td>
                    <td
                        style={cell_style(&id, "percent_change_1h", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_1h".to_string())
                        })}
                    >{ item.formatted_percent_change_1h() }</td>
                    <td
                        style={cell_style(&id, "percent_change_24h", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_24h".to_string())
                        })}
                    >{ item.formatted_percent_change_24h() }</td>
                    <td
                        style={cell_style(&id, "percent_change_7d", &model.selected_cells)}
                        onclick={link.callback({
                            let id = id.clone();
                            move |_| Msg::ToggleCellSelection(id.clone(), "percent_change_7d".to_string())
                        })}
                    >{ item.formatted_percent_change_7d() }</td>
                    <td
                        style={cell_style(&id, "volume24", &model.selected_cells)}
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

