use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SortOrder {
    None,
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn toggle(&self) -> SortOrder {
        match self {
            SortOrder::None => SortOrder::Ascending,
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            SortOrder::None => "black",
            SortOrder::Ascending => "green",
            SortOrder::Descending => "red",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub label: String,
    pub sort_order: SortOrder,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(HeaderButton)]
pub fn header_button(props: &HeaderProps) -> Html {
    let HeaderProps { label, sort_order, onclick } = props.clone();
    let style = format!("color: {}", sort_order.color());

    html! {
        <th style={style} onclick={onclick}>{ label }</th>
    }
}

