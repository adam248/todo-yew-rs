use yew::prelude::*;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub id: usize,
    pub text: String,
}

#[function_component(Task)]
pub fn task(props: &Props) -> Html {
    let _on_delete = Callback::from(|id| id);
    let _on_edit = Callback::from(|id| id);
    html! {
        <li><input type="checkbox"/> {&props.text}{" "}<button>{"✏️"}</button><button>{"🗑️"}</button></li>
    }
}
