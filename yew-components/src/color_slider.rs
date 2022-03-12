use num::traits::AsPrimitive;
use num::Float;
use vek::ColorComponent;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod color_slider_truck_app;
use color_slider_truck_app::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod> {
    #[prop_or_default]
    pub onchange: Callback<f64>,
    #[prop_or(None)]
    pub step: Option<f64>,
    #[prop_or(0.0)]
    pub value: f64,
    pub color_start: vek::Rgba<T>,
    pub color_end: vek::Rgba<T>,
    #[prop_or(true)]
    pub linear: bool,
}

#[function_component(ColorSlider)]
pub fn color_slider<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod>(
    props: &Props<T>,
) -> Html {
    let oninput = {
        let onchange = props.onchange.clone();
        Callback::from(move |evt: InputEvent| {
            let value = evt
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .map(|i| i.value())
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or_default();
            onchange.emit(value);
        })
    };

    let truck_props = ColorSliderTruckProps {
        color_start: props.color_start,
        color_end: props.color_end,
        linear: props.linear,
    };

    let color = if props.linear {
        vek::Rgba::lerp(
            props.color_start.map(|x| x.as_()),
            props.color_end.map(|x| x.as_()),
            props.value as f32,
        )
    } else {
        let color_start = props.color_start.map(|x| x.as_().powf(1.0 / 2.2));
        let color_end = props.color_end.map(|x| x.as_().powf(1.0 / 2.2));
        vek::Rgba::lerp(color_start, color_end, props.value as f32).map(|x| x.powf(2.2))
    };
    let r = color.r * 255.0;
    let g = color.g * 255.0;
    let b = color.b * 255.0;

    let rate = props.value;
    let percentage = rate * 100.0;
    let css = css! {r#"
        width: 100%;
        flex-grow: 1;
        display: flex;
        flex-direction: row;
        align-items: center;

        & .slider {
            width: 64px;
            height: 16px;
            flex-grow: 1;
            position: relative;

            & > canvas{
                width: 100%;
                height: 8px;
                position: absolute;
                margin: auto;
                top: 0;
                bottom: 0;
                border-radius: 4px;
            }

            & .handle {
                width: 12px;
                height: 12px;
                margin: auto;
                top: 0;
                bottom: 0;
                border: 1px solid #eeee;
                border-radius: 50%;
                position: absolute;
                filter: drop-shadow(0 0 6px rgba(0, 0, 0, .9));
                transition: all 0.5s;
            }
        }

        & input[type="range"] {
            appearance: none;
            width: 100%;
            height: 100%;
            display: block;
            position: relative;
            background: transparent;

            &::-webkit-slider-thumb {
                opacity: 0;
                cursor: pointer;
            }

            &::-moz-range-thumb {
                opacity: 0;
                cursor: pointer;
            }
        }

        & input[type="number"] {
            appearance: none;
            -moz-appearance: textfield;
            border: none;
            background: #ccc;
            border-radius: 8px;
            width: 64px;
            height: 16px;
            margin-left: 8px;
            text-align: center;
            display: block;

            &::-webkit-outer-spin-button, &::-webkit-inner-spin-button {
                appearance: none;
            }
        }
    "#};
    let dynamic_css = dynamic_css!(format! {r#"
        & .handle {{
            left: calc({percentage}% - 12px * {rate});
            background: rgb({r}, {g}, {b});
        }}
    "#});

    html! {
        <div class={classes!(css, dynamic_css)}>
            <div class="slider">
                <WgpuCanvas<ColorSliderTruckApp<T>> animated=false props={truck_props}/>
                <div class="handle"/>
                <input type="range"
                    min="0" max="1"
                    step={props.step.map(|step| step.to_string())}
                    value={props.value.to_string()}
                    oninput={oninput.clone()}/>
            </div>
            <input type="number"
                min="0" max="1"
                step={props.step.map(|step| step.to_string())}
                value={props.value.to_string()}
                {oninput}/>
        </div>
    }
}
