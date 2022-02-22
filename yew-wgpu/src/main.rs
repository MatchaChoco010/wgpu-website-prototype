mod hooks;
mod my_canvas_app;
mod pass;
mod rgba_slider;
mod wgpu_canvas;

use my_canvas_app::*;
use rgba_slider::*;
use wgpu_canvas::*;
use yew::prelude::*;

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

    let state = MyCanvasAppState { clear_color: color };

    html! {
        <>
            <WgpuCanvas<MyCanvasApp> {state} />
            <RgbSlider {color} {onchange} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<MyApp>();
}
