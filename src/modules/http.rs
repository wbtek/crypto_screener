
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

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_wasm_bindgen::from_value;

use crate::json::*;

// Struct to hold the response from the API
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    data: Vec<CoinData>,
}

// Struct to hold individual coin data
#[derive(Debug, Serialize, Deserialize)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    nameid: String,
    #[serde(deserialize_with = "string_or_number")]
    rank: String,
    #[serde(deserialize_with = "string_or_float")]
    price_usd: String,
    #[serde(deserialize_with = "string_or_float")]
    percent_change_1h: String,
    #[serde(deserialize_with = "string_or_float")]
    percent_change_24h: String,
    #[serde(deserialize_with = "string_or_float")]
    percent_change_7d: String,
    #[serde(deserialize_with = "string_or_float")]
    market_cap_usd: String,
    #[serde(deserialize_with = "string_or_float")]
    volume24: String,
    #[serde(deserialize_with = "string_or_float")]
    volume24a: String,
    #[serde(deserialize_with = "string_or_float")]
    csupply: String,
    #[serde(deserialize_with = "string_or_float")]
    tsupply: String,
    msupply: Option<String>,
}

#[wasm_bindgen]
pub async fn fetch_data(api_url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(api_url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    // let json_str = js_sys::JSON::stringify(&json).unwrap();
    // log(&format!("Raw JSON response: {}", json_str));
    Ok(json)
}

#[wasm_bindgen]
pub fn display_data(json: &JsValue) {
    match from_value::<ApiResponse>(json.clone()) {
        Ok(api_response) => {
            log(&format!("{:<10} {:<20} {:<15} {:<10} {:<10} {:<10}",
                "Symbol", "Name", "Price (USD)", "1h %", "24h %", "7d %"));

            for value in &api_response.data {
                log(&format!("{:<10} {:<20} {:<15} {:<10} {:<10} {:<10}",
                    value.symbol,
                    value.name,
                    value.price_usd,
                    value.percent_change_1h,
                    value.percent_change_24h,
                    value.percent_change_7d));
            }
        },
        Err(err) => {
            log(&format!("Failed to deserialize JSON: {:?}", err));
        }
    }
}

#[wasm_bindgen]
extern {
    fn log(s: &str);
}

