use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew_components::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod my_canvas_app;
mod pass;

use my_canvas_app::*;

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

    let css = css!(
        "
        width: 100%;
        min-height: 100%;
        padding: 16px;
        background: #101010;
        color: #ccc;

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

        & .resize {
            width: 400px;
            height: 200px;
            overflow: hidden;
            resize: both;

            & > canvas {
                width: 100%;
                height: 100%;
            }
        }

        & .resize-elem {
            width: 100px;
            height: 12px;
            overflow: hidden;
            resize: both;
        }
        "
    );
    html! {
        <div class={css}>
            <div class="resize">
                <WgpuCanvas<MyCanvasApp> {props} animated={false}/>
            </div>
            <div class="layout-row">
                <div class="layout-column">
                    <div class="layout-row controls">
                        <Slider min={-1.0} max={2.0} value={slider_value}
                            onchange={Callback::from(move |v| slider_value_state.set(v))}/>
                    </div>
                    <div class="layout-row">
                        <div class="resize-elem"/>
                        <div>{"Background Color: "}</div>
                        <ColorPicker {color} {onchange}/>
                    </div>
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
