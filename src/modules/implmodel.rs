
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

//! # Model Implementation Module
//!
//! This module provides additional methods for the `Model` struct, extending its functionality
//! for managing and interacting with the application's state in the WBTek Crypto Screener.
//! It includes utility methods that help with sorting, selection, and other data handling tasks
//! within the main application model.
//!
//! ## Key Methods
//!
//! - `sort_order`: Determines the current sort order for a specified column.
//!
//! These methods are designed to work seamlessly with the rest of the application by encapsulating
//! state management logic within the `Model` struct, making it easier to maintain and modify behavior
//! related to sorting, data processing, and user interactions.

use super::button::SortOrder;
use super::component::Model;

impl Model {
    /// Determines the current sort order for a specified column.
    ///
    /// This method checks if the specified `column` is the one currently being sorted in the model.
    /// If so, it returns the appropriate sort order (`Ascending` or `Descending`) based on the
    /// `sort_asc` field. If the column is not currently sorted, it returns `SortOrder::None`.
    ///
    /// # Parameters
    ///
    /// - `column`: The name of the column to check for the current sort order.
    ///
    /// # Returns
    ///
    /// Returns a `SortOrder` indicating the current sorting state:
    /// - `SortOrder::Ascending` if `column` matches the currently sorted column and `sort_asc` is `true`.
    /// - `SortOrder::Descending` if `column` matches the currently sorted column and `sort_asc` is `false`.
    /// - `SortOrder::None` if `column` does not match the currently sorted column.
    ///
    /// # Example
    ///
    /// ```rust
    /// let model = Model {
    ///     sort_by: Some("price_usd".to_string()),
    ///     sort_asc: true,
    ///     ..Default::default()
    /// };
    /// assert_eq!(model.sort_order("price_usd"), SortOrder::Ascending);
    /// assert_eq!(model.sort_order("symbol"), SortOrder::None);
    /// ```
    pub fn sort_order(&self, column: &str) -> SortOrder {
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
}
