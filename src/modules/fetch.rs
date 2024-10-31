
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

//! # Data Fetching Module
//!
//! This module provides functionality to fetch cryptocurrency data from an external API.
//! The `fetch_data` function asynchronously requests data and parses it into a collection
//! of `CryptoData` structs for use within the application.

use reqwest::Client;
use serde_json::Value;
use super::cryptodata::CryptoData;

/// Fetches cryptocurrency data from an external API and parses it into a vector of `CryptoData` structs.
///
/// This function uses `reqwest` to send an HTTP GET request to a cryptocurrency API,
/// retrieves JSON data, and attempts to deserialize it into a list of `CryptoData` entries.
/// If the data is unavailable or does not contain expected fields, an empty vector is returned.
///
/// # Returns
///
/// - `Ok(Vec<CryptoData>)`: A vector of `CryptoData` instances if the data is fetched and parsed successfully.
/// - `Err(reqwest::Error)`: An error if the request or JSON parsing fails.
///
/// # Example
///
/// ```rust
/// let data = fetch_data().await?;
/// for crypto in data {
///     println!("{:?}", crypto);
/// }
/// ```
///
/// # Errors
///
/// Returns a `reqwest::Error` if the HTTP request or JSON deserialization fails.
///
/// # API Endpoint
///
/// The function currently fetches data from:
/// `https://api.coinlore.net/api/tickers/`. It retrieves general market information
/// for various cryptocurrencies. An alternative API URL is commented out in the code.
pub async fn fetch_data() -> Result<Vec<CryptoData>, reqwest::Error> {
    let client = Client::new();
    let res = client
        // .get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd")
        .get("https://api.coinlore.net/api/tickers/")
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Extracts and parses the "data" field into `CryptoData` structs if available.
    if let Some(array) = res.get("data").and_then(|d| d.as_array()) {
        let data: Vec<CryptoData> = array
            .iter()
            .filter_map(|item| serde_json::from_value(item.clone()).ok())
            .collect();
        Ok(data)
    } else {
        // Returns an empty vector if "data" is missing or has invalid format
        Ok(Vec::new())
    }
}
