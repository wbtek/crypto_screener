
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

use serde::{Deserialize, Deserializer, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_wasm_bindgen::from_value;

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

fn string_or_float<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrFloatVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrFloatVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string, a float, or null")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_unit<E>(self) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok("".to_string())
        }
    }

    deserializer.deserialize_any(StringOrFloatVisitor)
}

fn string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string, a number, or null")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_unit<E>(self) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok("".to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
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
    let json_str = js_sys::JSON::stringify(&json).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_fetch_data() {
        let api_url = "https://api.coinlore.net/api/tickers/";
        let result = fetch_data(api_url).await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_display_data() {
        let coin_data = CoinData {
            id: "90".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            nameid: "bitcoin".to_string(),
            rank: "1".to_string(),
            price_usd: "45000.0".to_string(),
            percent_change_1h: "0.5".to_string(),
            percent_change_24h: "2.0".to_string(),
            percent_change_7d: "5.0".to_string(),
            market_cap_usd: "850000000000".to_string(),
            volume24: "35000000000".to_string(),
            volume24a: "32000000000".to_string(),
            csupply: "18000000".to_string(),
            tsupply: "21000000".to_string(),
            msupply: Some("21000000".to_string()),
        };

        let api_response = ApiResponse { data: vec![coin_data] };

        display_data(&JsValue::from_serde(&api_response).unwrap());
    }
}

