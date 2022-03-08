use num::Float;
use num_traits::FromPrimitive;
use std::fmt::Display;
use vek::ColorComponent;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, HtmlElement};
use yew::prelude::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod hsv_palette_app;
use hsv_palette_app::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct Props<T: Float + ColorComponent> {
    #[prop_or_default]
    pub onchange: Callback<vek::Rgba<T>>,
    pub color: vek::Rgba<T>,
}

#[function_component(HsvPalette)]
pub fn hsv_palettel<
    T: 'static + Float + FromPrimitive + Display + ColorComponent + bytemuck::Pod,
>(
    props: &Props<T>,
) -> Html {
    let div_ref = use_node_ref();
    let onmousemove_closure_state: Rc<RefCell<Option<Closure<dyn Fn(MouseEvent)>>>> =
        use_mut_ref(|| None);

    let app_props = HsvPaletteProps { color: props.color };
    let hsl = crate::util::linear_rgb_to_hsl(props.color.rgb());
    let hsv = crate::util::linear_rgb_to_hsv(props.color.rgb());

    let h = hsl.x;
    let radius = 64.0 * 0.9;
    let s_percentage = hsl.y * T::from_f64(100.0).unwrap();
    let l_percentage = hsl.z * T::from_f64(100.0).unwrap();
    let edge = T::from_f64(128.0 * 0.8 / 2.0.sqrt()).unwrap();
    let half_edge = edge / T::from_f64(2.0).unwrap();
    let edge_s = edge * hsv.y;
    let edge_v = edge * hsv.z;

    let onmousedown = {
        let div_ref = div_ref.clone();
        let onmousemove_closure_state = onmousemove_closure_state.clone();
        let onchange = props.onchange.clone();
        let h = h;
        let s = hsv.y;
        let v = hsv.z;
        let a = props.color.a;
        Callback::from(move |evt: MouseEvent| {
            evt.prevent_default();
            evt.stop_propagation();

            let client_x = evt.client_x() as f64;
            let client_y = evt.client_y() as f64;
            let div = div_ref.cast::<HtmlElement>().unwrap();
            let rect = div.get_bounding_client_rect();
            let pos = vek::Vec2::new(client_x - rect.left(), client_y - rect.top());
            let pos = pos - vek::Vec2::one() * 64.0;
            let edge = 128.0 * 0.8 / 2.0.sqrt();
            let half_edge = edge / 2.0;
            let magnitude = (pos / 64.0).magnitude();

            if pos.x.abs() < half_edge && pos.y.abs() < half_edge {
                let pos = vek::Vec2::new(pos.x + half_edge, -pos.y + half_edge) / edge;
                let s = T::from_f64(pos.x).unwrap();
                let v = T::from_f64(pos.y).unwrap();
                let h = h;
                let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(h, s, v));
                let rgba = vek::Rgba::new(rgb.r, rgb.g, rgb.b, a);
                onchange.emit(rgba);

                let onmousemove_palette_closure = {
                    let div_ref = div_ref.clone();
                    let onchange = onchange.clone();
                    Closure::wrap(Box::new(move |evt: MouseEvent| {
                        evt.prevent_default();
                        let client_x = evt.client_x() as f64;
                        let client_y = evt.client_y() as f64;
                        let div = div_ref.cast::<HtmlElement>().unwrap();
                        let rect = div.get_bounding_client_rect();
                        let pos = vek::Vec2::new(client_x - rect.left(), client_y - rect.top());
                        let pos = pos - vek::Vec2::one() * 64.0;
                        let edge = 128.0 * 0.8 / 2.0.sqrt();
                        let half_edge = edge / 2.0;
                        let pos = vek::Vec2::new(pos.x + half_edge, -pos.y + half_edge) / edge;
                        let s = T::from_f64(pos.x.clamp(0.0, 1.0)).unwrap();
                        let v = T::from_f64(pos.y.clamp(0.0, 1.0)).unwrap();
                        let h = h;
                        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(h, s, v));
                        let rgba = vek::Rgba::new(rgb.r, rgb.g, rgb.b, a);
                        onchange.emit(rgba);
                    }) as Box<dyn Fn(MouseEvent)>)
                };

                let document = web_sys::window().and_then(|w| w.document()).unwrap();
                document
                    .add_event_listener_with_callback(
                        "mousemove",
                        onmousemove_palette_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                *onmousemove_closure_state.borrow_mut() = Some(onmousemove_palette_closure);
            }

            if 0.82 < magnitude && magnitude < 0.98 {
                let rad = (-pos.y).atan2(pos.x);
                let deg = (-rad.to_degrees() - 90.0 + 360.0) % 360.0;
                let s = s;
                let v = v;
                let h = T::from_f64(deg).unwrap();
                let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(h, s, v));
                let rgba = vek::Rgba::new(rgb.r, rgb.g, rgb.b, a);
                onchange.emit(rgba);

                let onmousemove_ring_closure = {
                    let div_ref = div_ref.clone();
                    let onchange = onchange.clone();
                    Closure::wrap(Box::new(move |evt: MouseEvent| {
                        evt.prevent_default();
                        let client_x = evt.client_x() as f64;
                        let client_y = evt.client_y() as f64;
                        let div = div_ref.cast::<HtmlElement>().unwrap();
                        let rect = div.get_bounding_client_rect();
                        let pos = vek::Vec2::new(client_x - rect.left(), client_y - rect.top());
                        let pos = pos - vek::Vec2::one() * 64.0;
                        let rad = (-pos.y).atan2(pos.x);
                        let deg = (-rad.to_degrees() - 90.0 + 360.0) % 360.0;
                        let s = s;
                        let v = v;
                        let h = T::from_f64(deg).unwrap();
                        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(h, s, v));
                        let rgba = vek::Rgba::new(rgb.r, rgb.g, rgb.b, a);
                        onchange.emit(rgba);
                    }) as Box<dyn Fn(MouseEvent)>)
                };

                let document = web_sys::window().and_then(|w| w.document()).unwrap();
                document
                    .add_event_listener_with_callback(
                        "mousemove",
                        onmousemove_ring_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                *onmousemove_closure_state.borrow_mut() = Some(onmousemove_ring_closure);
            }

            let onmouseup_closure = {
                let onmousemove_closure_state = onmousemove_closure_state.clone();
                Closure::wrap(Box::new(move |evt: MouseEvent| {
                    evt.prevent_default();
                    let document = web_sys::window().and_then(|w| w.document()).unwrap();
                    if let Some(closure) = onmousemove_closure_state.borrow_mut().take() {
                        document
                            .remove_event_listener_with_callback(
                                "mousemove",
                                closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                }) as Box<dyn Fn(MouseEvent)>)
            };

            let mut once_option = AddEventListenerOptions::new();
            once_option.once(true);
            let document = web_sys::window().and_then(|w| w.document()).unwrap();
            document
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "mouseup",
                    onmouseup_closure.as_ref().unchecked_ref(),
                    &once_option,
                )
                .unwrap();

            onmouseup_closure.forget();
        })
    };

    let css = css! {"
        width: 128px;
        height: 128px;
        position: relative;

        & > canvas {
            width: 100%;
            height: 100%;
            position: absolute;
            display: block;
        }

        & > .ring-handle {
            width: 12px;
            height: 12px;
            position: absolute;
            margin: auto;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            cursor: pointer;
            border: 1px solid #eeee;
            border-radius: 50%;
            filter: drop-shadow(0 0 6px rgba(0, 0, 0, .9));
            transition: all 0.5s;
        }

        & > .palette-handle {
            width: 12px;
            height: 12px;
            position: absolute;
            margin: auto;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            cursor: pointer;
            border: 1px solid #eeee;
            border-radius: 50%;
            filter: drop-shadow(0 0 3px rgba(0, 0, 0, .9));
            transition: all 0.5s;
        }
    "};
    let dynamic_css = dynamic_css! {format!{r#"
        & > .ring-handle {{
            background: hsl({h}, 100%, 50%);
            transform: rotate({h}deg) translate(0, {radius});
        }}
        & > .palette-handle {{
            background: hsl({h}, {s_percentage}%, {l_percentage}%);
            transform: translate({edge_s}px, -{edge_v}px) translate(-{half_edge}px, {half_edge}px);
        }}
    "#}};
    html! {
        <div class={classes!(css, dynamic_css)} ref={div_ref} {onmousedown}>
            <WgpuCanvas<HsvPaletteApp<T>> animated=false props={app_props}/>
            <div class="ring-handle"/>
            <div class="palette-handle"/>
        </div>
    }
}