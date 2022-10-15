use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MyObject {
    user: String,
    fav: String,
}

#[function_component(HelloWorldApp)]
pub fn hello_world_app() -> Html {
    let name = "Adam";
    let myo = MyObject {
        user: "Adam".into(),
        fav: "Rust".into(),
    };
    log!(name);
    log!(serde_json::to_string_pretty(&myo).unwrap());
    html! {
        <>
        <h1>{"Hello world!"}</h1>
        <p>{"Hello "}{name}{"!"}</p>
        </>
    }
}
