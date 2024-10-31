
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

//! # Sorting Module
//!
//! This module provides sorting functionality for the cryptocurrency data in the WBTek Crypto Screener
//! application. The main function, `sort_data`, allows sorting of `CryptoData` entries based on a selected
//! column in either ascending or descending order. The module includes specific comparison functions
//! to handle different data types and ensure consistent ordering across various fields.

use std::cmp::Ordering;
use super::cryptodata::CryptoData;

/// Sorts the `data` array of `CryptoData` items by the specified column and order.
///
/// This function uses a match statement to select the appropriate comparison function
/// based on the `sort_by` field. Each comparison function handles a specific field in `CryptoData`,
/// such as `symbol`, `name`, `price_usd`, etc. After sorting, the data can be reversed
/// if `sort_asc` is set to `false`, making the order descending.
///
/// # Parameters
///
/// - `data`: A mutable slice of `CryptoData` items to be sorted.
/// - `sort_by`: An optional `String` specifying the field to sort by (e.g., "symbol" or "price_usd").
/// - `sort_asc`: A boolean that determines the sort order:
///   - `true` for ascending order.
///   - `false` for descending order.
///
/// # Example
///
/// ```rust
/// sort_data(&mut data, &Some("price_usd".to_string()), true);
/// ```
pub fn sort_data(data: &mut [CryptoData], sort_by: &Option<String>, sort_asc: bool) {
    if let Some(ref sort_by) = sort_by {
        match sort_by.as_str() {
            "symbol" => data.sort_by(compare_symbol),
            "name" => data.sort_by(compare_name),
            "price_usd" => data.sort_by(compare_price_usd),
            "percent_change_1h" => data.sort_by(compare_percent_change_1h),
            "percent_change_24h" => data.sort_by(compare_percent_change_24h),
            "percent_change_7d" => data.sort_by(compare_percent_change_7d),
            "volume24" => data.sort_by(compare_volume24),
            _ => {},
        }
        if !sort_asc {
            data.reverse();
        }
    }
}

/// Compares two optional `String` values as `f64`, used for numeric sorting (e.g., price).
///
/// - Returns `Ordering::Equal` if either value is `None` or cannot be parsed.
fn compare_f64(a: &Option<String>, b: &Option<String>) -> Ordering {
    let a_val = a.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let b_val = b.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    a_val.partial_cmp(&b_val).unwrap_or(Ordering::Equal)
}

/// Compares two optional `String` values as percentages (parsed as `f64`).
///
/// - Returns `Ordering::Equal` if parsing fails for either value.
fn compare_percent(a: &Option<String>, b: &Option<String>) -> Ordering {
    let a_val = a.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let b_val = b.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    a_val.partial_cmp(&b_val).unwrap_or(Ordering::Equal)
}

/// Compares two optional `f64` values, used for numeric sorting (e.g., volume).
///
/// - Returns `Ordering::Equal` if either value is `None`.
fn compare_f64_opt(a: &Option<f64>, b: &Option<f64>) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

/// Compares two `CryptoData` items by symbol (case-insensitive).
///
/// Returns an ordering based on the lexicographic order of `symbol`.
fn compare_symbol(a: &CryptoData, b: &CryptoData) -> Ordering {
    a.symbol.as_ref().map(|s| s
        .to_lowercase()).unwrap_or_default()
        .cmp(
            &b.symbol.as_ref().map(|s| s
                .to_lowercase()).unwrap_or_default(),
        )
}

/// Compares two `CryptoData` items by name (case-insensitive).
///
/// Returns an ordering based on the lexicographic order of `name`.
fn compare_name(a: &CryptoData, b: &CryptoData) -> Ordering {
    a.name.as_ref().map(|s| s
        .to_lowercase()).unwrap_or_default()
        .cmp(
            &b.name.as_ref().map(|s| s
                .to_lowercase()).unwrap_or_default(),
        )
}

/// Compares two `CryptoData` items by price in USD.
///
/// Uses `compare_f64` to handle the optional `price_usd` field.
fn compare_price_usd(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_f64(&a.price_usd, &b.price_usd)
}

/// Compares two `CryptoData` items by 1-hour percent change.
///
/// Uses `compare_percent` for parsing and comparison.
fn compare_percent_change_1h(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_1h, &b.percent_change_1h)
}

/// Compares two `CryptoData` items by 24-hour percent change.
///
/// Uses `compare_percent` for parsing and comparison.
fn compare_percent_change_24h(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_24h, &b.percent_change_24h)
}

/// Compares two `CryptoData` items by 7-day percent change.
///
/// Uses `compare_percent` for parsing and comparison.
fn compare_percent_change_7d(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_7d, &b.percent_change_7d)
}

/// Compares two `CryptoData` items by 24-hour trading volume.
///
/// Uses `compare_f64_opt` to handle the optional `volume24` field.
fn compare_volume24(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_f64_opt(&a.volume24, &b.volume24)
}
