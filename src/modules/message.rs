
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

//! # Message Definitions for Model Component
//!
//! This module defines the `Msg` enum, which represents the different types of messages
//! that can be sent to update the state of the `Model` component in the WBTek Crypto Screener.
//! These messages trigger specific actions, such as fetching data, sorting columns, or toggling
//! UI elements like the "About" modal.

use super::cryptodata::CryptoData;

/// Enum representing messages that can modify the application's state.
///
/// Each variant of `Msg` corresponds to a specific action, with associated data
/// if needed to carry out the operation.
pub enum Msg {
    /// Initiates a request to fetch cryptocurrency data.
    FetchData,

    /// Sets the fetched data to the model or stores an error message if the request fails.
    ///
    /// - `Ok(Vec<CryptoData>)`: Successfully retrieved data.
    /// - `Err(reqwest::Error)`: An error occurred during the data fetch.
    SetData(Result<Vec<CryptoData>, reqwest::Error>),

    /// Sorts the data by the specified column.
    ///
    /// - `String`: The name of the column to sort by (e.g., "price_usd").
    SortBy(String),

    /// Toggles selection state for a specific cell in the data table.
    ///
    /// - `String`: A unique identifier for the row (e.g., a cryptocurrency ID).
    /// - `String`: The name of the column (e.g., "symbol").
    ToggleCellSelection(String, String),

    /// Toggles the visibility of the "About" modal.
    ToggleAbout,
}
