
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

use crate::modules::sort::sort_data;
use crate::modules::but::SortOrder;
use crate::ui::component::Model;

impl Model {
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

    pub fn handle_sort(&mut self, column: String) {
        if self.sort_by.as_ref() == Some(&column) {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort_by = Some(column);
            self.sort_asc = true;
        }
        sort_data(&mut self.data, &self.sort_by, self.sort_asc);
    }

    pub fn toggle_cell_selection(&mut self, id: String, column: String) {
        let cell = (id.clone(), column.clone());
        if self.selected_cells.contains(&cell) {
            self.selected_cells.remove(&cell);
        } else {
            self.selected_cells.insert(cell.clone());
        }
    }

    pub fn cell_style(&self, id: &str, column: &str) -> String {
        if self.selected_cells.contains(&(id.to_string(), column.to_string())) {
            "background-color: steelblue;".to_string()
        } else {
            "".to_string()
        }
    }
}

