use yew_color_picker::*;
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

    let resize_canvas_css = css!(
        "
        width: 150px;
        height: 150px;
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
        display: flex;
        align-items: center;
        padding: 1rem;

        & > div {
            font-size: 1.2rem;
        }
        "
    );
    html! {
        <>
            <div class={resize_canvas_css}>
                <WgpuCanvas<MyCanvasApp> {props} animated={false}/>
            </div>
            <div class={css}>
                <div>{"Background Color: "}</div>
                <ColorPicker {color} {onchange}/>
            </div>
        </>
    }
}

#[wasm_bindgen(start)]
#[no_mangle]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}
