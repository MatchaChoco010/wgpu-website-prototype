use yew_components::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod my_canvas_app;
mod pass;

use my_canvas_app::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(MyApp)]
fn my_app() -> Html {
    let color_state = use_state(|| vek::Rgba::new(1.0, 1.0, 0.0, 0.5));
    let color = *color_state;

    let onchange = Callback::from({
        let color_state = color_state.clone();
        move |rgba| {
            color_state.set(rgba);
        }
    });

    let props = MyCanvasAppProps { clear_color: color };

    let slider_value_state = use_state(|| 0.0);
    let slider_value = *slider_value_state;

    let color_slider_value_state = use_state(|| 0.5);
    let color_slider_value = *color_slider_value_state;

    let resize_canvas_css = css!(
        "
        width: 400px;
        height: 200px;
        overflow: hidden;
        resize: both;

        & > canvas {
            width: 100%;
            height: 100%;
        }
        "
    );
    let css = css!(
        "
        width: 100%;
        height: 100%;
        padding: 16px;
        background: #101010;

        & > :not(:first-child) {
            margin-top: 32px;
        }

        & .layout-column {
            display: flex;
            flex-direction: column;

            & > :not(:first-child) {
                margin-top: 16px;
            }
        }

        & .layout-row {
            display: flex;
            align-items: center;
            flex-direction: row;

            & > :not(:first-child) {
                margin-left: 12px;
            }
        }

        & .controls {
            width: 256px;
            color: #eee;
            font-size: 1rem;
        }
        "
    );
    html! {
        <div class={css}>
            <div class={resize_canvas_css}>
                <WgpuCanvas<MyCanvasApp> {props} animated={false}/>
            </div>
            <div class="layout-column">
                <div class="layout-row controls">
                    <Slider min={-1.0} max={2.0} step={0.01} value={slider_value}
                        onchange={Callback::from(move |v| slider_value_state.set(v))}/>
                </div>
                <div class="layout-row controls">
                    <ColorSlider<f32> step={0.01} value={color_slider_value}
                        color_start={vek::Rgba::new(0.0, 0.0, 0.0, 1.0)}
                        color_end={vek::Rgba::new(1.0, 1.0, 1.0, 1.0)}
                        onchange={Callback::from(move |v| color_slider_value_state.set(v))}/>
                </div>
                <div class="layout-row controls">
                    <ColorSlider<f32> step={0.01} value={color_slider_value}
                        color_start={vek::Rgba::new(0.0, 0.0, 0.0, 1.0)}
                        color_end={vek::Rgba::new(1.0, 1.0, 1.0, 1.0)}
                        linear=false/>
                </div>
                <div class="layout-row controls">
                    <HsvPalette<f32> {color} onchange={onchange.clone()}/>
                </div>
                <div class="layout-row controls">
                    <div>{"Background Color: "}</div>
                    <ColorPicker {color} {onchange}/>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
#[no_mangle]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}
