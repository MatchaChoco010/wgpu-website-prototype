use gloo::events::EventListener;
use gloo::render::request_animation_frame;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_style_in_rs::*;
use yew_wgpu::*;

mod toggle_app;
use toggle_app::*;

use crate::dropdown::*;
use crate::hsv_palette::*;
use crate::hsva_sliders::*;
use crate::rgba_sliders::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<vek::Rgba<f32>>,
    pub color: vek::Rgba<f32>,
    #[prop_or(true)]
    pub alpha: bool,
}

#[function_component(ColorPicker)]
pub fn color_picker(props: &Props) -> Html {
    let hover_item_ref = use_node_ref();

    let hover_display = use_state(|| "display: none;");
    let hover_show_state = use_state(|| None);
    let hover_anim_ref = use_mut_ref(|| None);
    let click_document_listener = use_mut_ref(|| None);

    let slider_class_state = use_mut_ref(|| "hsv");
    let dropdown = vec!["HSV", "RGB"];
    let dropdown_value = use_state(|| 0);

    let hover_left_position = use_state(|| 0.0);
    let hover_before_left_position = use_state(|| 0.0);
    let hover_bottom = use_state(|| true);

    // Open Hover Card
    let onmousedown = Callback::from({
        let hover_item_ref = hover_item_ref.clone();
        let hover_display = hover_display.clone();
        let hover_show_state = hover_show_state.clone();
        let click_document_listener = click_document_listener.clone();

        move |_| {
            if (*hover_show_state).is_some() {
                log::debug!("Hi");
                return;
            }

            hover_display.set("display: flex;");

            let anim = request_animation_frame({
                let hover_item_ref = hover_item_ref.clone();
                let hover_show_state = hover_show_state.clone();
                let hover_display = hover_display.clone();
                let click_document_listener = click_document_listener.clone();
                move |_| {
                    hover_show_state.set(Some("show"));

                    let document = gloo::utils::document();
                    let listener = EventListener::new(&document, "mousedown", {
                        let hover_item = hover_item_ref.cast::<HtmlElement>().unwrap();
                        let hover_display = hover_display.clone();
                        let click_document_listener = click_document_listener.clone();
                        let hover_show_state = hover_show_state.clone();
                        move |evt| {
                            if let Some(target) = evt.target().unwrap().dyn_ref::<HtmlElement>() {
                                if hover_item.contains(Some(target)) {
                                    return;
                                }

                                hover_show_state.set(None);

                                let hover_display = hover_display.clone();
                                EventListener::once(&hover_item, "transitionend", move |_| {
                                    hover_display.set("display: none;");
                                })
                                .forget();

                                if let Some(listener) = click_document_listener.take() {
                                    drop(listener);
                                }
                            }
                        }
                    });
                    *click_document_listener.borrow_mut() = Some(listener);
                }
            });
            *hover_anim_ref.borrow_mut() = Some(anim);
        }
    });

    // Change HSV/RGb
    let dropdown_onchange = Callback::from({
        let slider_class_state = slider_class_state.clone();
        let dropdown_value = dropdown_value.clone();
        move |index| {
            if index == 0 {
                *slider_class_state.borrow_mut() = "hsv";
                dropdown_value.set(0);
            } else {
                *slider_class_state.borrow_mut() = "rgb";
                dropdown_value.set(1);
            }
        }
    });

    // Check Hover Card Intersection
    {
        let hover_item_ref = hover_item_ref.clone();
        let hover_left_position = hover_left_position.clone();
        let hover_before_left_position = hover_before_left_position.clone();
        let hover_bottom = hover_bottom.clone();
        {
            use_effect_with_deps(
                move |hover_item_ref: &NodeRef| {
                    let body = gloo::utils::body();
                    let hover_item = hover_item_ref.cast::<web_sys::Element>().unwrap();
                    let f = Closure::wrap(Box::new({
                        let hover_item = hover_item.clone();
                        let hover_left_position = hover_left_position.clone();
                        let hover_before_left_position = hover_before_left_position.clone();
                        let hover_bottom = hover_bottom.clone();
                        move || {
                            let parent_rect = hover_item
                                .parent_element()
                                .unwrap()
                                .get_bounding_client_rect();
                            let body_rect = body.get_bounding_client_rect();
                            let left = 0.0_f64
                                .max(
                                    -((parent_rect.left() + parent_rect.right()) / 2.0
                                        - (180.0 + 12.0)
                                        - body_rect.left()),
                                )
                                .min(
                                    -((parent_rect.left() + parent_rect.right()) / 2.0
                                        + (180.0 + 12.0)
                                        - body_rect.right()),
                                );
                            hover_left_position.set(left);
                            hover_before_left_position
                                .set(-left.clamp(-(180.0 - 16.0), 180.0 - 16.0));

                            hover_bottom.set(body_rect.bottom() - parent_rect.bottom() > 150.0);
                        }
                    }) as Box<dyn FnMut()>);
                    let observer =
                        web_sys::ResizeObserver::new(f.as_ref().unchecked_ref()).unwrap();
                    observer.observe(&hover_item);
                    f.forget();
                    move || observer.disconnect()
                },
                hover_item_ref,
            );
        };
    }

    let toggle_props = ToggleProps { color: props.color };

    let alpha = props.alpha;
    let color = props.color;
    let onchange = props.onchange.clone();
    let show = *hover_show_state;
    let slider_class = *slider_class_state.borrow();
    let hover_left = *hover_left_position;
    let hover_before_left = *hover_before_left_position;
    let hover_top_bottom_class = if *hover_bottom { "bottom" } else { "top" };
    let display = *hover_display;

    let css = css! {r#"
        width: 32px;
        height: 16px;

        & > canvas {
            width: 100%;
            height: 100%;
            display: block;
            cursor: pointer;
            background: #0e0e0e;
            border-radius: 8px;
        }

        & > .hover {
            position: relative;
            margin-left: calc(-180px + 16px);
            width: 360px;
            height: 150px;
            background: #292929;
            border-radius: 8px;
            box-shadow: 0 0 20px 15px #1a1a1a inset;
            z-index: 10;
            display: none;
            opacity: 0;
            visibility: hidden;
            filter: drop-shadow(0 0 0 rgba(0, 0, 0, 0));

            transition: opacity 0.5s, visibility 0.5s, filter 0.5s;

            &.bottom {
                top: 16px;

                &::before {
                    content: "";
                    position: absolute;
                    top: -12px;
                    margin-left: calc(180px - 12px);
                    border: 12px solid transparent;
                    border-top: 0;
                    border-bottom: 12px solid #1a1a1a;
                }
            }

            &.top {
                bottom: calc(16px + 16px + 150px);

                &::before {
                    content: "";
                    position: absolute;
                    bottom: -12px;
                    margin-left: calc(180px - 12px);
                    border: 12px solid transparent;
                    border-top: 12px solid #1a1a1a;
                    border-bottom: 0;
                }
            }

            &.show {
                opacity: 1;
                visibility: visible;
                filter: drop-shadow(0 10px 15px #000);
            }

            & > .palette {
                width: 140px;
                height: 140px;
                margin: auto 8px;
            }

            & > .right-area {
                flex-grow: 1;
                display: flex;
                flex-direction: column;
                margin: auto;
                margin-right: 16px;

                & > .dropdown {
                    display: flex;
                    justify-content: flex-end;
                    margin-bottom: 4px;
                }

                & > .slider {
                    width: 100%;
                    position: relative;
                    background: #111;
                    padding: 8px;
                    border-radius: 8px;
                }

                & > .hsv > div:nth-child(2) {
                    display: none;
                    position: absolute;
                    top: 0;
                    left: 0;
                }

                & > .rgb > div:nth-child(1) {
                    display: none;
                    position: absolute;
                    top: 0;
                    left: 0;
                }
            }
        }
    "#};
    let dynamic_css = dynamic_css! {format!{r#"
        & > .hover {{
            left: {hover_left}px;
            {display}

            &::before {{
                left: {hover_before_left}px;
            }}
        }}
    "#}};
    html! {
        <div class={classes!(css, dynamic_css)}>
            <WgpuCanvas<ToggleApp> animated=false props={toggle_props} {onmousedown}/>
            <div class={classes!("hover", show, hover_top_bottom_class)} ref={hover_item_ref}>
                <div class="palette">
                    <HsvPalette<f32> {color} onchange={onchange.clone()}/>
                </div>
                <div class="right-area">
                    <div class="dropdown">
                        <Dropdown list={dropdown} value={*dropdown_value}
                            onchange={dropdown_onchange} />
                    </div>
                    <div class={classes!("slider", slider_class)}>
                        <HsvaSliders<f32> {color} onchange={onchange.clone()} {alpha}/>
                        <RgbaSliders<f32> {color} onchange={onchange.clone()} {alpha}/>
                    </div>
                </div>
            </div>
        </div>
    }
}
