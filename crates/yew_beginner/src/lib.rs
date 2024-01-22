use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

mod components;

use components::{
    atoms::main_title::{Color, MainTitle},
    molecules::custom_form::CustomForm,
};

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let main_title_load = Callback::from(|message: String| log!(message));

    html! {
        <div class={stylesheet}>
            <MainTitle title="hi there!" color={Color::Normal} on_load={main_title_load} />
            <CustomForm />
            <p>{"more text"}</p>
        </div>
    }
}
