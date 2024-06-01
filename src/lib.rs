
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

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use js_sys::JSON;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
    total_volume: f64,
}

#[wasm_bindgen]
pub async fn get_crypto_data(_symbol: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=volume_desc&per_page=50&page=1");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let headers = Headers::new().unwrap();
    headers.set("Accept", "application/json")?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    
    let json_string = JSON::stringify(&json).unwrap().as_string().unwrap();
    let coins: Vec<Coin> = serde_json::from_str(&json_string).unwrap();

    let filtered_coins: Vec<Coin> = coins.into_iter()
        .filter(|coin| 
            coin.symbol != "dai" &&
            coin.symbol != "weth" && 
            !coin.symbol.contains("usd") && 
            !coin.name.contains("Wrapped") && 
            !coin.name.contains("Stable") )
        .collect();

    Ok(to_value(&filtered_coins).unwrap())
}

#[wasm_bindgen]
pub async fn get_filtered_data() -> Result<JsValue, JsValue> {
    let filtered_data = get_crypto_data("all").await?;
    Ok(filtered_data)
}

