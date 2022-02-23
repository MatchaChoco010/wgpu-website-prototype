use yew_color_picker::*;
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
    let color_state = use_state(|| vek::Rgba::black());
    let color = *color_state;

    let onchange = Callback::from({
        let color_state = color_state.clone();
        move |rgba| {
            color_state.set(rgba);
        }
    });

    let props = MyCanvasAppProps { clear_color: color };

    html! {
        <>
            <div id="resize-canvas">
                <WgpuCanvas<MyCanvasApp> {props} />
            </div>
            <RgbSlider {color} {onchange} />
        </>
    }
}

#[wasm_bindgen(start)]
#[no_mangle]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}
