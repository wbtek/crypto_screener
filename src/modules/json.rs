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

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CryptoData {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub price_usd: Option<String>,
    pub percent_change_1h: Option<String>,
    pub percent_change_24h: Option<String>,
    pub percent_change_7d: Option<String>,
    pub volume24: Option<f64>,
}

impl CryptoData {
    pub fn truncated_name(&self) -> String {
        self.name.clone().unwrap_or_default().chars().take(30).collect()
    }

    pub fn formatted_price(&self) -> String {
        if let Some(price) = &self.price_usd {
            if let Ok(p) = price.parse::<f64>() {
                return format!("{:.6}", p);
            }
        }
        self.price_usd.clone().unwrap_or_default()
    }

    pub fn formatted_percent_change_1h(&self) -> String {
        self.percent_change_1h.clone().unwrap_or_default()
    }

    pub fn formatted_percent_change_24h(&self) -> String {
        self.percent_change_24h.clone().unwrap_or_default()
    }

    pub fn formatted_percent_change_7d(&self) -> String {
        self.percent_change_7d.clone().unwrap_or_default()
    }

    pub fn formatted_volume(&self) -> String {
        self.volume24.map(|v| format!("{:.2}", v)).unwrap_or_default()
    }
}
