use yew::prelude::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod toggle_app;
use toggle_app::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<vek::Rgba<f32>>,
    pub color: vek::Rgba<f32>,
}

#[function_component(ColorPicker)]
pub fn color_picker(props: &Props) -> Html {
    let toggle = use_state(|| false);
    let onchange_toggle = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.set(!*toggle))
    };
    let display = if *toggle { "block" } else { "none" };

    let toggle_props = ToggleProps { color: props.color };

    let css = css!(
        "
        width: 64px;
        height: 32px;

        & > label {
            width: 100%;
            height: 100%;
            display: block;
            cursor: pointer;
        }

        & > label > input {
            display: none;
        }

        & > label > canvas {
            width: 100%;
            height: 100%;
            background: #0e0e0e;
            border: 4px #0e0e0e solid;
            border-radius: 8px;
        }

        & > div {
            background: cyan;
            position: relative;
            width: 256px;
            height: 128px;
        }
        "
    );
    let dynamic_css = dynamic_css!(format! {"
        & > div {{
            display: {display}
        }}
    "});
    html! {
        <div class={classes!(css, dynamic_css)}>
            <label>
                <input type="checkbox" onchange={onchange_toggle}/>
                <WgpuCanvas<ToggleApp> animated=true props={toggle_props}/>
            </label>
            <div>
                {"Hello yew-style-in-rs!"}
            </div>
        </div>
    }
}
