
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

//! # Header Button Module
//!
//! This module defines the `HeaderButton` component, a customizable table header button
//! for sorting in the WBTek Crypto Screener application. The button can be used to sort data 
//! in ascending or descending order, with different visual styles to indicate the sort direction.
//!
//! ## Components
//!
//! - `SortOrder`: An enum representing the current sort state of the button (None, Ascending, or Descending).
//! - `HeaderProps`: A struct that holds properties for the `HeaderButton` component, including the 
//!   label text, sort order, and a callback function for handling click events.
//! - `header_button`: A function component in Yew that renders a `<th>` element styled according to the
//!   current sort state, and invokes a callback when clicked.

use yew::prelude::{Properties, Callback, MouseEvent, function_component, Html, html};

/// Represents the sorting state of a header button.
///
/// - `None`: No sorting applied.
/// - `Ascending`: Data is sorted in ascending order.
/// - `Descending`: Data is sorted in descending order.
///
/// The `SortOrder` enum allows the `HeaderButton` component to display
/// different visual cues depending on the current sort order.
#[derive(Clone, PartialEq)]
pub enum SortOrder {
    None,
    Ascending,
    Descending,
}

impl SortOrder {
    /// Generates a CSS class name based on the current sort order.
    ///
    /// This class name can be applied to elements to change their appearance
    /// depending on the sort order. If `SortOrder` is `None`, an empty string
    /// is returned, which does not apply any styling.
    ///
    /// # Returns
    ///
    /// - `""` for `None`
    /// - `"ascending"` for `Ascending`
    /// - `"descending"` for `Descending`
    ///
    /// # Examples
    ///
    /// ```
    /// let order = SortOrder::Ascending;
    /// assert_eq!(order.class(), "ascending");
    /// ```
    pub fn class(&self) -> &'static str {
        match self {
            SortOrder::None => "",
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        }
    }
}

/// Properties for the `HeaderButton` component.
///
/// `HeaderProps` defines the label text, the current sort order, and a
/// callback function to handle click events. These properties are passed 
/// to `HeaderButton` to configure its appearance and behavior.
///
/// - `label`: The text to display on the button.
/// - `sort_order`: The current sort order for this header button (None, Ascending, or Descending).
/// - `onclick`: A callback triggered when the button is clicked.
#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    /// Text label displayed on the header button.
    pub label: String,

    /// Current sort order for the button (None, Ascending, Descending).
    pub sort_order: SortOrder,

    /// Callback function triggered on click events.
    pub onclick: Callback<MouseEvent>,
}

/// A Yew function component that renders a table header button for sorting.
///
/// `HeaderButton` is a customizable table header button used in sortable tables
/// to visually indicate the sorting direction. When clicked, it triggers a callback
/// to update the sorting state. The button applies different CSS classes to reflect
/// the sort order, which can be styled accordingly.
///
/// # Parameters
///
/// - `props`: [`HeaderProps`] containing the label, sort order, and a click event callback.
///
/// # HTML Structure
///
/// The rendered HTML includes a `<th>` element with dynamic class names based on `sort_order`.
/// When clicked, the `onclick` callback is triggered.
///
/// # Examples
///
/// ```rust
/// let props = HeaderProps {
///     label: "Name".to_string(),
///     sort_order: SortOrder::Ascending,
///     onclick: Callback::noop(),
/// };
/// let header_button = header_button(&props);
/// ```
#[function_component(HeaderButton)]
pub fn header_button(props: &HeaderProps) -> Html {
    let HeaderProps { label, sort_order, onclick } = props.clone();
    let class = format!("header-button {}", sort_order.class());

    html! {
        <th class={class} onclick={onclick}>{ label }</th>
    }
}