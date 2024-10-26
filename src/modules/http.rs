
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

use reqwest::Client;
use serde_json::Value;
use crate::json::CryptoData;

pub async fn fetch_data() -> Result<Vec<CryptoData>, reqwest::Error> {
    let client = Client::new();
    let res = client
        // .get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd")
        .get("https://api.coinlore.net/api/tickers/")
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(array) = res.get("data").and_then(|d| d.as_array()) {
        let data: Vec<CryptoData> = array
            .iter()
            .filter_map(|item| serde_json::from_value(item.clone()).ok())
            .collect();
        Ok(data)
    } else {
        Ok(Vec::new())
    }
}

