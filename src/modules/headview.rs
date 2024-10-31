
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

//! # Header View Module
//!
//! This module provides the `view_header` function, which generates a sortable table header 
//! for the WBTek Crypto Screener application. The header displays column labels for various 
//! cryptocurrency attributes and allows users to click on these headers to sort the data.

use yew::prelude::{Context, Html, html};
use super::button::HeaderButton;
use super::component::Model;
use super::message::Msg;

/// Renders a table header with sortable columns for cryptocurrency data.
///
/// This function creates a row of `HeaderButton` components representing different data columns
/// (such as "Symbol," "Name," "Price (USD)," and percentage changes over various periods). Each
/// button displays a label and triggers a sorting action when clicked, updating the application's 
/// sorting state based on the selected column.
///
/// # Parameters
///
/// - `ctx`: Reference to the Yew [`Context`] for the main application [`Model`], which allows
///   interaction with component links for handling click events.
/// - `underscore_line`: A string of underscores used as a visual separator under the header row.
/// - `model`: A reference to the main application model, used to retrieve the current sorting order
///   for each column.
///
/// # Returns
///
/// Returns an [`Html`] fragment containing the table header with sortable columns.
///
/// # Column Definitions
///
/// Each `HeaderButton` represents a column in the table and includes:
/// - A label indicating the column's name.
/// - A `sort_order` representing the current sorting state for the column (ascending or descending).
/// - An `onclick` callback that sends a `Msg::SortBy` message to update the sort order.
///
/// # Example
///
/// ```rust
/// let underscore_line = "_".repeat(130);
/// let header = view_header(ctx, &underscore_line, model);
/// ```

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
