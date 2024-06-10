// message.rs

use crate::json::CryptoData;
use crate::ui::drag::DragMsg;

pub enum Msg {
    FetchData,
    SetData(Result<Vec<CryptoData>, reqwest::Error>),
    SortBy(String),
    ToggleCellSelection(String, String),
    DragEvent(i32, i32), // Add DragEvent here
}

impl From<DragMsg> for Msg {
    fn from(drag_msg: DragMsg) -> Self {
        match drag_msg {
            DragMsg::DragEvent(x, y) => Msg::DragEvent(x, y),
        }
    }
}
