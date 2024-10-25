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

#[derive(Clone, PartialEq)]
pub enum SortOrder {
    None,
    Ascending,
    Descending,
}

// Make string to match CSS to set button color
impl SortOrder {
    pub fn class(&self) -> &'static str {
        match self {
            SortOrder::None => "",
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub label: String,
    pub sort_order: SortOrder,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(HeaderButton)]
pub fn header_button(props: &HeaderProps) -> Html {
    let HeaderProps { label, sort_order,  onclick } = props.clone();
    let class = format!("header-button {}", sort_order.class());

    html! {
        <th class={class} onclick={onclick}>{ label }</th>
    }
}

