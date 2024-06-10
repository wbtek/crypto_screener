// drag.rs

use yew::prelude::*;
use yew::html::Scope;
use crate::modules::ui::component::Model;
use crate::modules::ui::message::Msg;

pub struct Drag {
    link: Scope<Model>,
}

pub enum DragMsg {
    DragEvent(i32, i32),
}

#[derive(Properties, Clone)]
pub struct DragProps {
    pub link: Scope<Model>,
}

impl PartialEq for DragProps {
    fn eq(&self, _: &Self) -> bool {
        true // Implement custom equality logic if needed
    }
}

impl Component for Drag {
    type Message = DragMsg;
    type Properties = DragProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.props().link.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DragMsg::DragEvent(x, y) => {
                self.link.send_message(Msg::DragEvent(x, y));
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div
                onmousedown={self.link.callback(|e: MouseEvent| DragMsg::DragEvent(e.client_x(), e.client_y()))}
                onmousemove={self.link.callback(|e: MouseEvent| DragMsg::DragEvent(e.client_x(), e.client_y()))}
                onmouseup={self.link.callback(|e: MouseEvent| DragMsg::DragEvent(e.client_x(), e.client_y()))}
            >
                { "Drag Area" }
            </div>
        }
    }
}
