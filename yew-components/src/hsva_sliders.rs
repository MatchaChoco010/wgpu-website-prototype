use num::traits::{AsPrimitive, FromPrimitive};
use num::Float;
use std::fmt::Display;
use vek::ColorComponent;
use yew::prelude::*;
use yew_style_in_rs::*;

use crate::color_slider::*;
use crate::hue_slider::*;

#[derive(Properties, PartialEq)]
pub struct Props<T: Float + ColorComponent> {
    #[prop_or_default]
    pub onchange: Callback<vek::Rgba<T>>,
    pub color: vek::Rgba<T>,
    #[prop_or(true)]
    pub alpha: bool,
}

#[function_component(HsvaSliders)]
pub fn hsva_sliders<
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
    let hsv = crate::util::linear_rgb_to_hsv(color.rgb());

    let h = hsv.x;
    let h_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        let hsv = hsv;
        move |h| {
            let hsv = vek::Vec3::new(T::from_f64(h).unwrap(), hsv.y, hsv.z);
            let rgb = crate::util::hsv_to_linear_rgb(hsv);
            onchange.emit(vek::Rgba::new(rgb.r, rgb.g, rgb.b, color.a));
        }
    });

    let s = hsv.y;
    let s_start = {
        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(hsv.x, T::zero(), hsv.z));
        vek::Rgba::new(rgb.r, rgb.g, rgb.b, T::one())
    };
    let s_end = {
        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(hsv.x, T::one(), hsv.z));
        vek::Rgba::new(rgb.r, rgb.g, rgb.b, T::one())
    };
    let s_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        let hsv = hsv;
        move |s| {
            let hsv = vek::Vec3::new(hsv.x, T::from_f64(s).unwrap(), hsv.z);
            let rgb = crate::util::hsv_to_linear_rgb(hsv);
            onchange.emit(vek::Rgba::new(rgb.r, rgb.g, rgb.b, color.a));
        }
    });

    let v = hsv.z;
    let v_start = {
        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(hsv.x, hsv.y, T::zero()));
        vek::Rgba::new(rgb.r, rgb.g, rgb.b, T::one())
    };
    let v_end = {
        let rgb = crate::util::hsv_to_linear_rgb(vek::Vec3::new(hsv.x, hsv.y, T::one()));
        vek::Rgba::new(rgb.r, rgb.g, rgb.b, T::one())
    };
    let v_onchange = Callback::from({
        let onchange = props.onchange.clone();
        let color = props.color;
        let hsv = hsv;
        move |v| {
            let hsv = vek::Vec3::new(hsv.x, hsv.y, T::from_f64(v).unwrap());
            let rgb = crate::util::hsv_to_linear_rgb(hsv);
            onchange.emit(vek::Rgba::new(rgb.r, rgb.g, rgb.b, color.a));
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
            color: #ccc;
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
                <span>{"H:"}</span>
                <HueSlider value={AsPrimitive::<f64>::as_(h)} onchange={h_onchange}/>
            </div>
            <div class="layout-row">
                <span>{"S:"}</span>
                <ColorSlider<T> color_start={s_start} color_end={s_end} linear=false
                    value={AsPrimitive::<f64>::as_(s)}
                    onchange={s_onchange}/>
            </div>
            <div class="layout-row">
                <span>{"V:"}</span>
                <ColorSlider<T> color_start={v_start} color_end={v_end} linear=false
                    value={AsPrimitive::<f64>::as_(v)}
                    onchange={v_onchange}/>
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
