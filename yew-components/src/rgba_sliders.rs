use num::traits::{AsPrimitive, FromPrimitive};
use num::Float;
use std::fmt::Display;
use vek::ColorComponent;
use yew::prelude::*;
use yew_style_in_rs::*;

use crate::color_slider::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: Float + ColorComponent> {
    #[prop_or_default]
    pub onchange: Callback<vek::Rgba<T>>,
    pub color: vek::Rgba<T>,
    #[prop_or(true)]
    pub alpha: bool,
}

#[function_component(RgbaSliders)]
pub fn rgba_sliders<
    T: 'static
        + Float
        + FromPrimitive
        + AsPrimitive<f32>
        + AsPrimitive<f64>
        + Display
        + ColorComponent
        + bytemuck::Pod,
>(
    props: &Props<T>,
) -> Html {
    let color = props.color;

    let r = color.r;
    let r_start = vek::Rgba::new(T::zero(), color.g, color.b, T::one());
    let r_end = vek::Rgba::new(T::one(), color.g, color.b, T::one());
    let r_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        move |r| {
            let mut c = color;
            c.r = T::from_f64(r).unwrap();
            onchange.emit(c);
        }
    });

    let g = color.g;
    let g_start = vek::Rgba::new(color.r, T::zero(), color.b, T::one());
    let g_end = vek::Rgba::new(color.r, T::one(), color.b, T::one());
    let g_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        move |g| {
            let mut c = color;
            c.g = T::from_f64(g).unwrap();
            onchange.emit(c);
        }
    });

    let b = color.b;
    let b_start = vek::Rgba::new(color.r, color.g, T::zero(), T::one());
    let b_end = vek::Rgba::new(color.r, color.g, T::one(), T::one());
    let b_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        move |b| {
            let mut c = color;
            c.b = T::from_f64(b).unwrap();
            onchange.emit(c);
        }
    });

    let a = color.a;
    let a_start = vek::Rgba::new(color.r, color.g, color.b, T::zero());
    let a_end = vek::Rgba::new(color.r, color.g, color.b, T::one());
    let a_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        move |a| {
            let mut c = color;
            c.a = T::from_f64(a).unwrap();
            onchange.emit(c);
        }
    });

    let css = css!(
        "
        width: 100%;
        height: 100%;

        & > :not(:first-child) {
            margin-top: 8px;
        }

        & span {
            display: block;
            font-size: 12px;
            width: 12px;
            margin: auto;
            top: 0;
            bottom: 0;
        }

        & .layout-row {
            display: flex;
            align-items: center;
            flex-direction: row;
        }
        "
    );
    html! {
        <div class={css}>
            <div class="layout-row">
                <span>{"R:"}</span>
                <ColorSlider<T> color_start={r_start} color_end={r_end}
                    value={AsPrimitive::<f64>::as_(r)}
                    onchange={r_onchange}/>
            </div>
            <div class="layout-row">
                <span>{"G:"}</span>
                <ColorSlider<T> color_start={g_start} color_end={g_end}
                    value={AsPrimitive::<f64>::as_(g)}
                    onchange={g_onchange}/>
            </div>
            <div class="layout-row">
                <span>{"B:"}</span>
                <ColorSlider<T> color_start={b_start} color_end={b_end}
                    value={AsPrimitive::<f64>::as_(b)}
                    onchange={b_onchange}/>
            </div>
            <div class="layout-row">
                <span>{"A:"}</span>
                <ColorSlider<T> color_start={a_start} color_end={a_end}
                    value={AsPrimitive::<f64>::as_(a)}
                    onchange={a_onchange} disable={!props.alpha}/>
            </div>
        </div>
    }
}
