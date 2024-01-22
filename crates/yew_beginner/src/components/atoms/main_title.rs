//! 自定义风格

use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub color: Option<Color>,
    pub on_load: Option<Callback<String>>,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let color = &props.color.clone().expect("msg").to_string();
    let title = &props.title.clone().unwrap();

    let stylesheet = style!(
        r#"
            .normal {
                color: white;
            }

            .ok {
                color: green;
            }

            .error {
                color: red;
            }
        "#
    )
    .unwrap();

    props.on_load.clone().unwrap().emit("I loaded".to_owned());

    html! {
        <div class={stylesheet}>
            <h1 class={color}>{title}</h1>
        </div>
    }
}
