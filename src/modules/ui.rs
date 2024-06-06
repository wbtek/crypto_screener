use yew::prelude::*;
use crate::modules::json::CryptoData;
use crate::modules::sort::sort_data;
use crate::modules::but::{HeaderButton, SortOrder};
use wasm_bindgen::prelude::*;

struct Model {
    data: Vec<CryptoData>,
    error_message: Option<String>,
    sort_by: Option<String>,
    sort_asc: bool,
}

pub enum Msg {
    FetchData,
    SetData(Result<Vec<CryptoData>, reqwest::Error>),
    SortBy(String),
}

impl Model {
    fn sort_order(&self, column: &str) -> SortOrder {
        if let Some(ref sort_by) = self.sort_by {
            if sort_by == column {
                if self.sort_asc {
                    return SortOrder::Ascending;
                } else {
                    return SortOrder::Descending;
                }
            }
        }
        SortOrder::None
    }

    fn handle_sort(&mut self, column: String) {
        if self.sort_by.as_ref() == Some(&column) {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort_by = Some(column);
            self.sort_asc = true;
        }
        sort_data(&mut self.data, &self.sort_by, self.sort_asc);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchData);
        Self { data: Vec::new(), error_message: None, sort_by: None, sort_asc: true }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let data = crate::modules::http::fetch_data().await;
                    link.send_message(Msg::SetData(data));
                });
                false
            }
            Msg::SetData(result) => {
                match result {
                    Ok(data) => {
                        self.data = data;
                        self.error_message = None;
                        sort_data(&mut self.data, &self.sort_by, self.sort_asc);
                    },
                    Err(err) => {
                        self.error_message = Some(format!("Failed to fetch data: {:?}", err));
                    },
                }
                true
            }
            Msg::SortBy(column) => {
                self.handle_sort(column);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let underscore_line = "_".repeat(130);

        html! {
            <div>
                <h1>{ "Crypto Screener" }</h1>
                { if let Some(error) = &self.error_message {
                    html! { <p style="color: red;">{ error }</p> }
                } else {
                    html! {}
                }}
                <table>
                    <thead>
                        <tr>
                            <HeaderButton label={"Symbol".to_string()}
                               sort_order={self.sort_order("symbol")}
                                  onclick={link.callback(|_| Msg::SortBy("symbol".to_string()))} />
                            <HeaderButton label={"Name".to_string()}
                               sort_order={self.sort_order("name")}
                                  onclick={link.callback(|_| Msg::SortBy("name".to_string()))} />
                            <HeaderButton label={"Price (USD)".to_string()}
                               sort_order={self.sort_order("price_usd")}
                                  onclick={link.callback(|_| Msg::SortBy("price_usd".to_string()))} />
                            <HeaderButton label={"1h %".to_string()}
                               sort_order={self.sort_order("percent_change_1h")}
                                  onclick={link.callback(|_| Msg::SortBy("percent_change_1h".to_string()))} />
                            <HeaderButton label={"24h %".to_string()}
                               sort_order={self.sort_order("percent_change_24h")}
                                  onclick={link.callback(|_| Msg::SortBy("percent_change_24h".to_string()))} />
                            <HeaderButton label={"7d %".to_string()}
                               sort_order={self.sort_order("percent_change_7d")}
                                  onclick={link.callback(|_| Msg::SortBy("percent_change_7d".to_string()))} />
                            <HeaderButton label={"Volume ($)".to_string()}
                               sort_order={self.sort_order("volume24")}
                                  onclick={link.callback(|_| Msg::SortBy("volume24".to_string()))} />
                        </tr>
                        <tr>
                            <th colspan="7" style="text-align: left;">{ underscore_line.clone() }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for self.data.iter().map(|item| {
                            html! {
                                <tr>
                                    <td style="text-align: left;">{ item.symbol.clone().unwrap_or_default() }</td>
                                    <td style="text-align: left;">{ item.truncated_name() }</td>
                                    <td style="text-align: right;">{ item.formatted_price() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_1h() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_24h() }</td>
                                    <td style="text-align: right;">{ item.formatted_percent_change_7d() }</td>
                                    <td style="text-align: right;">{ item.formatted_volume() }</td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Model>::new().render();
}

