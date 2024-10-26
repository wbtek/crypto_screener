
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

use std::cmp::Ordering;
use super::json::CryptoData;

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

/*
fn compare<T: Ord>(a: &Option<T>, b: &Option<T>) -> Ordering {
    a.cmp(b)
}
*/

fn compare_f64(a: &Option<String>, b: &Option<String>) -> Ordering {
    let a_val = a.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let b_val = b.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    a_val.partial_cmp(&b_val).unwrap_or(Ordering::Equal)
}

fn compare_percent(a: &Option<String>, b: &Option<String>) -> Ordering {
    let a_val = a.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let b_val = b.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    a_val.partial_cmp(&b_val).unwrap_or(Ordering::Equal)
}

fn compare_f64_opt(a: &Option<f64>, b: &Option<f64>) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

fn compare_symbol(a: &CryptoData, b: &CryptoData) -> Ordering {
    a.symbol.as_ref().map(|s| s
        .to_lowercase()).unwrap_or_default()
        .cmp(
            &b.symbol.as_ref().map(|s| s
                .to_lowercase()).unwrap_or_default(),
        )
}

fn compare_name(a: &CryptoData, b: &CryptoData) -> Ordering {
    a.name.as_ref().map(|s| s
        .to_lowercase()).unwrap_or_default()
        .cmp(
            &b.name.as_ref().map(|s| s
                .to_lowercase()).unwrap_or_default(),
        )
}

fn compare_price_usd(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_f64(&a.price_usd, &b.price_usd)
}

fn compare_percent_change_1h(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_1h, &b.percent_change_1h)
}

fn compare_percent_change_24h(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_24h, &b.percent_change_24h)
}

fn compare_percent_change_7d(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_percent(&a.percent_change_7d, &b.percent_change_7d)
}

fn compare_volume24(a: &CryptoData, b: &CryptoData) -> Ordering {
    compare_f64_opt(&a.volume24, &b.volume24)
}

