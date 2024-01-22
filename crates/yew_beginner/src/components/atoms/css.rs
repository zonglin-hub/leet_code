//! 使用 `style` 宏定义样式

use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(AppCss)]
pub fn app_css() -> Html {
    let stylesheet = style! {
        r#"
            h1 {
                color: orange;
            }

            p {
                color: white;
            }
        "#
    }
    .unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{"hello world"}</h1>
            <p>{"more text"}</p>
            <p class={css!("color: red; font-size: 75px;")}>{"more text"}</p>
        </div>
    }
}
