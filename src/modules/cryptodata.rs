
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

//! # CryptoData Module
//!
//! This module defines the `CryptoData` struct, which represents a data record
//! for a cryptocurrency. It includes fields for various metrics such as price,
//! percent changes, and trading volume, and provides methods to format and
//! display these values in a user-friendly way.

use serde::Deserialize;

/// Represents cryptocurrency data, including name, symbol, price, percent change,
/// and 24-hour trading volume. This struct is designed to hold data parsed
/// from a JSON source, with all fields being optional to account for missing values.
#[derive(Deserialize, Debug)]
pub struct CryptoData {
    /// The symbol or ticker of the cryptocurrency (e.g., BTC for Bitcoin).
    pub symbol: Option<String>,
    
    /// The full name of the cryptocurrency (e.g., Bitcoin).
    pub name: Option<String>,
    
    /// The current price of the cryptocurrency in USD.
    pub price_usd: Option<String>,
    
    /// The percent change in price over the last hour.
    pub percent_change_1h: Option<String>,
    
    /// The percent change in price over the last 24 hours.
    pub percent_change_24h: Option<String>,
    
    /// The percent change in price over the last 7 days.
    pub percent_change_7d: Option<String>,
    
    /// The 24-hour trading volume of the cryptocurrency.
    pub volume24: Option<f64>,
}

impl CryptoData {
    /// Returns a truncated version of the cryptocurrency name, limited to 30 characters.
    ///
    /// If `name` is `None`, returns an empty string. This is useful for displaying
    /// abbreviated names in UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// let crypto = CryptoData { name: Some("Bitcoin".to_string()), ..Default::default() };
    /// assert_eq!(crypto.truncated_name(), "Bitcoin");
    /// ```
    pub fn truncated_name(&self) -> String {
        self.name.clone().unwrap_or_default().chars().take(30).collect()
    }

    /// Returns the price formatted to six decimal places if parsable as a `f64`.
    ///
    /// If the price cannot be parsed or is `None`, returns the original `price_usd`
    /// string value or an empty string if `price_usd` is also `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let crypto = CryptoData { price_usd: Some("1234.56789".to_string()), ..Default::default() };
    /// assert_eq!(crypto.formatted_price(), "1234.567890");
    /// ```
    pub fn formatted_price(&self) -> String {
        if let Some(price) = &self.price_usd {
            if let Ok(p) = price.parse::<f64>() {
                return format!("{:.6}", p);
            }
        }
        self.price_usd.clone().unwrap_or_default()
    }

    /// Returns the 1-hour percent change as a string, or an empty string if `None`.
    ///
    /// This method ensures a consistent return type for display purposes.
    pub fn formatted_percent_change_1h(&self) -> String {
        self.percent_change_1h.clone().unwrap_or_default()
    }

    /// Returns the 24-hour percent change as a string, or an empty string if `None`.
    ///
    /// This method ensures a consistent return type for display purposes.
    pub fn formatted_percent_change_24h(&self) -> String {
        self.percent_change_24h.clone().unwrap_or_default()
    }

    /// Returns the 7-day percent change as a string, or an empty string if `None`.
    ///
    /// This method ensures a consistent return type for display purposes.
    pub fn formatted_percent_change_7d(&self) -> String {
        self.percent_change_7d.clone().unwrap_or_default()
    }

    /// Returns the 24-hour trading volume formatted to two decimal places.
    ///
    /// If `volume24` is `None`, returns an empty string. This method provides
    /// a formatted representation of trading volume for display.
    ///
    /// # Examples
    ///
    /// ```
    /// let crypto = CryptoData { volume24: Some(12345.6789), ..Default::default() };
    /// assert_eq!(crypto.formatted_volume(), "12345.68");
    /// ```
    pub fn formatted_volume(&self) -> String {
        self.volume24.map(|v| format!("{:.2}", v)).unwrap_or_default()
    }
}
