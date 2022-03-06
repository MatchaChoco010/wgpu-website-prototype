use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
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
    let display = if *toggle { Some("show") } else { None };

    let toggle_props = ToggleProps { color: props.color };

    let css = css!(
        r#"
        width: 64px;
        height: 32px;

        & > label {
            width: 100%;
            height: 100%;
            display: block;
            cursor: pointer;

            & > input {
                display: none;
            }

            & > canvas {
                width: 100%;
                height: 100%;
                background: #0e0e0e;
                border: 4px #0e0e0e solid;
                border-radius: 8px;
            }
        }

        & > div {
            position: relative;
            top: 16px;
            left: -128px;
            width: 320px;
            height: 128px;
            background: #1f1f1f;
            border-radius: 8px;
            display: flex;
            opacity: 0;
            filter: drop-shadow(0 0 0 rgba(0, 0, 0, 0));
            transition: all 0.5s;

            &:before {
                content: "";
                position: absolute;
                top: -24px;
                left: 160px;
                margin-left: -15px;
                border: 12px solid transparent;
                border-bottom: 12px solid #1f1f1f;
            }

            &.show {
                opacity: 1;
                filter: drop-shadow(0 20px 10px rgba(0, 0, 0, .7));
            }

            & > canvas {
                width: 128px;
                height: 128px;
            }

            & > .rgb-slider {
                display: flex;
                flex-direction: column;
                flex-grow: 1;
                flex-shrink: 1;
                margin: 8px;

                & > .slider {
                    display: flex;
                    flex-direction: row;
                    align-items: center;

                    & > div {
                        color: #eee;
                        font-weight: bold;
                        font-size: 1rem;
                    }

                    & input[type="range"] {
                        width: 64px;
                        flex-grow: 1;
                    }

                    & input[type="number"] {
                        border: none;
                        background: #ccc;
                        border-radius: 4px;
                        width: 48px;
                        margin-left: 8px;
                        text-align: right;
                        display: block;

                        &:focus {
                            border: none;
                        }
                    }
                }
            }
        }
        "#
    );
    html! {
        <div class={css}>
            <label>
                <input type="checkbox" oninput={onchange_toggle}/>
                <WgpuCanvas<ToggleApp> animated=true props={toggle_props}/>
            </label>
            <div class={classes!("hover", display)}>
                <canvas/>
                <div class="rgb-slider">
                    <div class="slider">
                        <div>{"R: "}</div>
                        <input type="range" min=0 max=1 step=0.01
                            value={props.color.r.to_string()}
                            oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.r = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                        <input type="number" step=0.01
                            value={props.color.r.to_string()}
                            oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.r = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                    </div>
                    <div class="slider">
                        <div>{"G: "}</div>
                        <input type="range" min=0 max=1 step=0.01
                            value={props.color.g.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.g = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                        <input type="number" step=0.01
                            value={props.color.g.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.g = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                    </div>
                    <div class="slider">
                        <div>{"B: "}</div>
                        <input type="range" min=0 max=1 step=0.01
                            value={props.color.b.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.b = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                        <input type="number" step=0.01
                            value={props.color.b.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.b = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                    </div>
                    <div class="slider">
                        <div>{"A: "}</div>
                        <input type="range" min=0 max=1 step=0.01
                            value={props.color.a.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.a = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                        <input type="number" step=0.01
                            value={props.color.a.to_string()}
                             oninput={
                                let onchange = props.onchange.clone();
                                let color = props.color.clone();
                                Callback::from(move |evt: InputEvent| {
                                    let mut rgba = color;
                                    rgba.a = evt.target()
                                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                                        .map(|i| i.value())
                                        .and_then(|v| v.parse::<f32>().ok())
                                        .unwrap_or_default();
                                    onchange.emit(rgba);
                                })
                            }/>
                    </div>
                </div>
            </div>
        </div>
    }
}
