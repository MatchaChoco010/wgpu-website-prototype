mod util;

mod slider;
pub use slider::*;

mod color_slider;
pub use color_slider::*;

mod hue_slider;
pub use hue_slider::*;

mod rgba_sliders;
pub use rgba_sliders::*;

mod hsv_palette;
pub use hsv_palette::*;

mod color_picker;
pub use color_picker::*;

// #[function_component(RgbSlider)]
// pub fn rgb_slider(props: &Props) -> Html {
//     let r_slider_ref = use_node_ref();
//     let g_slider_ref = use_node_ref();
//     let b_slider_ref = use_node_ref();
//     let oninput = Callback::from({
//         let r_slider_ref = r_slider_ref.clone();
//         let g_slider_ref = g_slider_ref.clone();
//         let b_slider_ref = b_slider_ref.clone();
//         let onchange = props.onchange.clone();
//         move |_| {
//             let r = if let Some(input) = r_slider_ref.cast::<HtmlInputElement>() {
//                 input.value().parse::<f32>().unwrap_or_default()
//             } else {
//                 0.0
//             };
//             let g = if let Some(input) = g_slider_ref.cast::<HtmlInputElement>() {
//                 input.value().parse::<f32>().unwrap_or_default()
//             } else {
//                 0.0
//             };
//             let b = if let Some(input) = b_slider_ref.cast::<HtmlInputElement>() {
//                 input.value().parse::<f32>().unwrap_or_default()
//             } else {
//                 0.0
//             };
//             let rgba = vek::Rgba::new(r, g, b, 1.0);
//             onchange.emit(rgba);
//         }
//     });
//     html! {
//         <div>
//             <label for="r-slider">{"R"}</label>
//             <input type="range" id="r-slider" min=0 max=1 step=0.01 ref={r_slider_ref} oninput={oninput.clone()} value={props.color.r.to_string()}/>
//             <label for="g-slider">{"G"}</label>
//             <input type="range" id="b-slider" min=0 max=1 step=0.01 ref={g_slider_ref} oninput={oninput.clone()} value={props.color.g.to_string()}/>
//             <label for="b-slider">{"B"}</label>
//             <input type="range" id="b-slider" min=0 max=1 step=0.01 ref={b_slider_ref} oninput={oninput.clone()} value={props.color.b.to_string()}/>
//         </div>
//     }
// }
