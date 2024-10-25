
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
use crate::modules::but::HeaderButton;
use crate::modules::ui::component::Model;
use crate::modules::ui::message::Msg;

pub fn view_header(ctx: &Context<Model>, underscore_line: &str, model: &Model) -> Html {
    let link = ctx.link();
    html! {
        <>
            <tr>
                <HeaderButton
                    label={"Symbol".to_string()}
                    sort_order={model.sort_order("symbol")}
                    onclick={link.callback(|_| Msg::SortBy("symbol".to_string()))}
                />
                <HeaderButton
                    label={"Name".to_string()}
                    sort_order={model.sort_order("name")}
                    onclick={link.callback(|_| Msg::SortBy("name".to_string()))}
                />
                <HeaderButton
                    label={"Price (USD)".to_string()}
                    sort_order={model.sort_order("price_usd")}
                    onclick={link.callback(|_| Msg::SortBy("price_usd".to_string()))}
                />
                <HeaderButton
                    label={"1h %".to_string()}
                    sort_order={model.sort_order("percent_change_1h")}
                    onclick={link.callback(|_| Msg::SortBy("percent_change_1h".to_string()))}
                />
                <HeaderButton
                    label={"24h %".to_string()}
                    sort_order={model.sort_order("percent_change_24h")}
                    onclick={link.callback(|_| Msg::SortBy("percent_change_24h".to_string()))}
                />
                <HeaderButton
                    label={"7d %".to_string()}
                    sort_order={model.sort_order("percent_change_7d")}
                    onclick={link.callback(|_| Msg::SortBy("percent_change_7d".to_string()))}
                />
                <HeaderButton
                    label={"Volume ($)".to_string()}
                    sort_order={model.sort_order("volume24")}
                    onclick={link.callback(|_| Msg::SortBy("volume24".to_string()))}
                />
            </tr>
            <tr>
                <th colspan="7" style="text-align: left;">{ underscore_line }</th>
            </tr>
        </>
    }
}

