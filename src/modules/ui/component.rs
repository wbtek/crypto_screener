// component.rs

use yew::prelude::*;
use std::collections::HashSet;
use crate::modules::json::CryptoData;
use crate::modules::sort::sort_data;
use crate::modules::ui::message::Msg;
use crate::modules::ui::headview::view_header;
use crate::modules::ui::rowview::view_rows;
use crate::modules::ui::drag::Drag;
use std::sync::atomic::{AtomicUsize, Ordering};

static COMPONENT_INIT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Model {
    pub data: Vec<CryptoData>,
    pub error_message: Option<String>,
    pub sort_by: Option<String>,
    pub sort_asc: bool,
    pub selected_cells: HashSet<(String, String)>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let count = COMPONENT_INIT_COUNT.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            log::info!("Model component created: {}", count);
            ctx.link().send_message(Msg::FetchData); // Ensure this only runs once
        } else {
            log::warn!("Model component created multiple times: {}", count);
        }

        Self { 
            data: Vec::new(), 
            error_message: None, 
            sort_by: None, 
            sort_asc: true, 
            selected_cells: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                log::info!("Fetching data"); // Log when fetching data
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
            Msg::ToggleCellSelection(id, column) => {
                self.toggle_cell_selection(id, column);
                true
            }
            Msg::DragEvent(x, y) => {
                log::info!("DragEvent received: x={}, y={}", x, y);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                        { view_header(ctx, &underscore_line, self) }
                    </thead>
                    <tbody>
                        { view_rows(ctx, self) }
                    </tbody>
                </table>
                <Drag link={ctx.link().clone()} />
            </div>
        }
    }
}
