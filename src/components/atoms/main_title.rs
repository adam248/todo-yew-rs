use yew::prelude::*;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub title: Option<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    match &props.title {
        Some(title) => html! {
            <h1>{{title}}</h1>
        },
        None => html! {
            <h1>{"Hello World!"}</h1>
        },
    }
}
