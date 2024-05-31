
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
// use serde::Deserialize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn get_crypto_data(_symbol: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=volume_desc&per_page=10&page=1");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

#[wasm_bindgen]
pub async fn get_filtered_data() -> Result<JsValue, JsValue> {
    let filtered_data = get_crypto_data("all").await?;
    Ok(filtered_data)
}

