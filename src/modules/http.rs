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
use crate::json::{StringNumberFloat, string_number_float};

// Struct to hold the response from the API
#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<CoinData>,
}

// Struct to hold individual coin data
#[derive(Debug, Deserialize)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    nameid: String,
    #[serde(deserialize_with = "string_number_float")]
    rank: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    price_usd: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    percent_change_1h: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    percent_change_24h: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    percent_change_7d: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    market_cap_usd: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    volume24: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    volume24a: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    csupply: StringNumberFloat,
    #[serde(deserialize_with = "string_number_float")]
    tsupply: StringNumberFloat,
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

    Ok(json)
}

#[wasm_bindgen]
pub fn display_data(json: &JsValue) {
    match from_value::<ApiResponse>(json.clone()) {
        Ok(api_response) => {
            log(&format!("{:<10} {:<30} {:>20} {:>10} {:>10} {:>10} {:>20}",
                "Symbol", "Name", "Price (USD)", "1h %", "24h %", "7d %", "Volume ($)"));
            log(&"_".repeat(130)); // Adjust the length of the line as needed
            for value in api_response.data {
                let truncated_name = if value.name.len() > 30 {
                    format!("{}...", &value.name[..27])
                } else {
                    value.name.clone()
                };

                let formatted_price = match value.price_usd {
                    StringNumberFloat::Float(f) => format!("{:>15.8}", f),
                    _ => format!("{:>15}", value.price_usd),
                };

                let formatted_percent_change_1h = match value.percent_change_1h {
                    StringNumberFloat::Float(f) => format!("{:>7.2}", f),
                    _ => format!("{:>7}", value.percent_change_1h),
                };

                let formatted_percent_change_24h = match value.percent_change_24h {
                    StringNumberFloat::Float(f) => format!("{:>7.2}", f),
                    _ => format!("{:>7}", value.percent_change_24h),
                };

                let formatted_percent_change_7d = match value.percent_change_7d {
                    StringNumberFloat::Float(f) => format!("{:>7.2}", f),
                    _ => format!("{:>7}", value.percent_change_7d),
                };

                let formatted_volume = match value.volume24 {
                    StringNumberFloat::Float(f) => format!("{:>20.2}", f),
                    _ => format!("{:>20}", value.volume24),
                };

                log(&format!(
                    "{:<10} {:<30} {:>20} {:>10} {:>10} {:>10} {:>20}",
                    value.symbol,
                    truncated_name,
                    formatted_price,
                    formatted_percent_change_1h,
                    formatted_percent_change_24h,
                    formatted_percent_change_7d,
                    formatted_volume
                ));
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

