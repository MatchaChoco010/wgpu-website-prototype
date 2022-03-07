use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, HtmlElement, HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew_style_in_rs::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onchange: Callback<f64>,
    #[prop_or(0.0)]
    pub min: f64,
    #[prop_or(1.0)]
    pub max: f64,
    #[prop_or(0.01)]
    pub step: f64,
    #[prop_or(0.0)]
    pub value: f64,
}

#[function_component(Slider)]
pub fn slider(props: &Props) -> Html {
    let slider_ref = use_node_ref();
    let handle_ref = use_node_ref();
    let transition_state = use_mut_ref(|| "transition: all ease 0.5s;");
    let onmousemove_closure_state: Rc<RefCell<Option<Closure<dyn Fn(MouseEvent)>>>> =
        use_mut_ref(|| None);

    let onmousedown_slider = {
        let slider_ref = slider_ref.clone();
        let transition_state = transition_state.clone();
        let onchange = props.onchange.clone();
        let max = props.max;
        let min = props.min;
        Callback::from(move |evt: MouseEvent| {
            *transition_state.borrow_mut() = "transition: all ease 0.5s;";

            evt.prevent_default();
            let client_x = evt.client_x() as f64;
            let slider = slider_ref.cast::<HtmlElement>().unwrap();
            let rect = slider.get_bounding_client_rect();
            let rate = (client_x - rect.left()) / (rect.right() - rect.left());
            let value = (max - min) * rate + min;
            onchange.emit(value);
        })
    };
    let onmousedown_handle = {
        let slider_ref = slider_ref.clone();
        let onchange = props.onchange.clone();
        let transition_state = transition_state.clone();
        let onmousemove_closure_state = onmousemove_closure_state.clone();
        let max = props.max;
        let min = props.min;

        Callback::from(move |evt: MouseEvent| {
            let onmousemove_closure_state = onmousemove_closure_state.clone();
            *transition_state.borrow_mut() = "";

            evt.prevent_default();
            evt.stop_propagation();

            let onmousemove_closure = {
                let slider_ref = slider_ref.clone();
                let onchange = onchange.clone();
                Closure::wrap(Box::new(move |evt: MouseEvent| {
                    evt.prevent_default();
                    let client_x = evt.client_x() as f64;
                    let slider = slider_ref.cast::<HtmlElement>().unwrap();
                    let rect = slider.get_bounding_client_rect();
                    let rate =
                        ((client_x - rect.left()) / (rect.right() - rect.left())).clamp(0.0, 1.0);
                    let value = (max - min) * rate + min;
                    onchange.emit(value);
                }) as Box<dyn Fn(MouseEvent)>)
            };
            let onmouseup_closure = {
                let onmousemove_closure_state = onmousemove_closure_state.clone();
                Closure::wrap(Box::new(move |evt: MouseEvent| {
                    evt.prevent_default();
                    let document = web_sys::window().and_then(|w| w.document()).unwrap();
                    let closure = onmousemove_closure_state.borrow_mut().take().unwrap();
                    document
                        .remove_event_listener_with_callback(
                            "mousemove",
                            closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }) as Box<dyn Fn(MouseEvent)>)
            };

            let mut once_option = AddEventListenerOptions::new();
            once_option.once(true);

            let document = web_sys::window().and_then(|w| w.document()).unwrap();
            document
                .add_event_listener_with_callback(
                    "mousemove",
                    onmousemove_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "mouseup",
                    onmouseup_closure.as_ref().unchecked_ref(),
                    &once_option,
                )
                .unwrap();

            *onmousemove_closure_state.borrow_mut() = Some(onmousemove_closure);
            onmouseup_closure.forget();
        })
    };
    let oninput = {
        let onchange = props.onchange.clone();
        Callback::from(move |evt: InputEvent| {
            let value = evt
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .map(|i| i.value())
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or_default();
            onchange.emit(value);
        })
    };

    let rate = ((props.value - props.min) / (props.max - props.min)).clamp(0.0, 1.0) * 100.0;
    let transition = transition_state.borrow().to_string();
    let css = css!(
        r#"
        width: 100%;
        flex-grow: 1;
        display: flex;
        flex-direction: row;
        align-items: center;

        & input[type="number"] {
            border: none;
            background: #ccc;
            border-radius: 8px;
            width: 48px;
            height: 16px;
            margin-left: 8px;
            text-align: right;
            display: block;

            &:focus {
                border: none;
            }
        }

        & .slider {
            width: 64px;
            height: 16px;
            flex-grow: 1;
            position: relative;

            & .track {
                width: 100%;
                height: 2px;
                position: absolute;
                margin: auto;
                top: 0;
                bottom: 0;
                background: #eee;
                border-radius: 1px;

                &::before {
                    content: "";
                    height: 100%;
                    position: absolute;
                    background: #30ffff;
                    border-radius: 1px;
                    left: 0;
                }
            }

            & .handle {
                width: 12px;
                height: 12px;
                margin: auto;
                margin-left: -6px;
                top: 0;
                bottom: 0;
                border: 1px solid #eeee;
                border-radius: 6px;
                background: #30ffff;
                position: absolute;
                cursor: pointer;
                filter: drop-shadow(0 0 6px rgba(0, 0, 0, .9));
            }
        }
    "#
    );
    let dynamic_css = dynamic_css!(format! {r#"
        & .slider {{
            & .track::before {{
                width: {rate}%;
                {transition}
            }}

            & .handle {{
                left: {rate}%;
                {transition}
            }}
        }}
    "#});

    html! {
        <div class={classes!(css, dynamic_css)}>
            <div class="slider"
                ref={slider_ref}
                onmousedown={onmousedown_slider}>
                <div class="track"/>
                <div class="handle"
                    ref={handle_ref}
                    aria-valuemin={props.min.to_string()}
                    aria-valuemax={props.max.to_string()}
                    aria-valuenow={props.value.to_string()}
                    draggable="false"
                    role="slider"
                    tabindex=0
                    onmousedown={onmousedown_handle}/>
            </div>
            <input type="number"
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                value={props.value.to_string()}
                {oninput}/>
        </div>
    }
}
