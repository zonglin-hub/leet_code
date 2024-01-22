//! 学习使用 `html` 宏，`console::log`

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(HTML)]
pub fn html1() -> Html {
    fn list_to_html(list: Vec<&str>) -> Vec<Html> {
        list.iter().map(|item| html! {<li>{item}</li>}).collect::<Vec<Html>>()
    }

    let name = "Brooks";

    let my_object = MyObject { username: name.to_owned(), favorite_language: "Rust".to_owned() };

    log!("my name is", name);

    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class_title = "my_title";

    let message = Some("I am a message");

    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    html! {
        <>
            <h1 class={class_title}>{"Hello World"}</h1>

            if class_title != "my_title" {
                <p>{"Hi there!"}</p>
            } else {
                <p>{"I'm is my_title!"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"no message to see today"}</p>
            }

            <ul>
                {list_to_html(tasks)}
            </ul>
        </>
    }
}
