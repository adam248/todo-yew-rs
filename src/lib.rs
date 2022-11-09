use yew::prelude::*;

mod components;

use components::atoms::add_new::AddNew;
use components::atoms::main_title::MainTitle;
use components::atoms::task::Task;

#[function_component(ToDoApp)]
pub fn to_do_app() -> Html {
    html! {
        <div>
        <MainTitle title="Todo App in Yew.rs!"/>

        <AddNew/>
        {render_to_do_list(get_tasks())}

        </div>
    }
}

fn render_to_do_list(tasks: Vec<&str>) -> Html {
    html! {
            <ul style="list-style-type: none">
            {list_to_html(tasks)}
            </ul>
    }
}

fn list_to_html(list: Vec<&str>) -> Html {
    list.iter()
        .enumerate()
        .map(|(idx, item)| html! {<Task id={idx} text={item.to_string()}/>})
        .collect()
}

fn get_tasks<'a>() -> Vec<&'a str> {
    vec![
        "Make breakfast",
        "Feed the dog",
        "Do laundry",
        "Go on a run",
        "Clean the house",
        "Go to the grocery store",
        "Do some coding",
        "Read a book",
    ]
}
