use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub id: Option<String>,
}

#[function_component(AddNew)]
pub fn add_new(props: &Props) -> Html {
    html! {
        <span>
        <input type="text"/>
        <button>{"➕"}</button>
        </span>

    }
}
