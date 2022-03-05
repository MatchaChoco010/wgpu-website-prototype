// use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_style_in_rs::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<vek::Rgba<f32>>,
    pub color: vek::Rgba<f32>,
}

#[function_component(ColorPicker)]
pub fn color_picker() -> Html {
    let css = css!(
        r"
        width: 200px;
        height: 100px;
        background: cyan;
    "
    );
    html! {
        <>
            <button>{"click"}</button>
            <div class={css}>
                {"Hello yew-style-in-rs!"}
            </div>
        </>
    }
}
