
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

//! # Utility Functions for Cell Selection and Styling
//!
//! This module provides helper functions for managing the selection state and styling of
//! table cells in the WBTek Crypto Screener application. These utilities allow cells to be
//! toggled as selected or unselected and to apply a specific style when selected.

use std::collections::HashSet;

/// Toggles the selection state of a table cell.
///
/// Given a unique identifier (`id`) and a column name (`column`), this function either adds
/// the cell to the set of `selected_cells` if it is not already present, or removes it if
/// it is currently selected.
///
/// # Parameters
///
/// - `selected_cells`: A mutable reference to a `HashSet` containing the selected cells, represented
///   as `(id, column)` pairs.
/// - `id`: A unique identifier for the row, typically the symbol or another unique value.
/// - `column`: The column name associated with the cell to toggle.
///
/// # Example
///
/// ```rust
/// let mut selected_cells = HashSet::new();
/// toggle_cell_selection(&mut selected_cells, "BTC".to_string(), "price_usd".to_string());
/// assert!(selected_cells.contains(&("BTC".to_string(), "price_usd".to_string())));
/// ```
pub fn toggle_cell_selection(
    selected_cells: &mut HashSet<(String, String)>,
    id: String,
    column: String,
) {
    let cell = (id.clone(), column.clone());
    if selected_cells.contains(&cell) {
        selected_cells.remove(&cell);
    } else {
        selected_cells.insert(cell.clone());
    }
}

/// Generates a CSS style string for a table cell based on its selection state.
///
/// This function checks if a cell, identified by `id` and `column`, is present in `selected_cells`.
/// If it is, the function returns a CSS style string to apply a background color; otherwise,
/// it returns an empty string, resulting in no style.
///
/// # Parameters
///
/// - `id`: The unique identifier for the row, represented as a `&str`.
/// - `column`: The column name associated with the cell, represented as a `&str`.
/// - `selected_cells`: A `HashSet` containing selected cells, represented as `(id, column)` pairs.
///
/// # Returns
///
/// Returns a `String` containing a CSS style for the cell background color if it is selected,
/// or an empty string if it is not selected.
///
/// # Example
///
/// ```rust
/// let mut selected_cells = HashSet::new();
/// selected_cells.insert(("BTC".to_string(), "price_usd".to_string()));
/// let style = cell_style("BTC", "price_usd", &selected_cells);
/// assert_eq!(style, "background-color: steelblue;");
/// ```
pub fn cell_style(id: &str, column: &str, selected_cells: &HashSet<(String, String)>) -> String {
    if selected_cells.contains(&(id.to_string(), column.to_string())) {
        "background-color: steelblue;".to_string()
    } else {
        "".to_string()
    }
}
