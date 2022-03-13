use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_style_in_rs::*;

#[derive(Properties)]
pub struct Props {
    #[prop_or_default]
    pub onchange: Callback<usize>,
    #[prop_or(0)]
    pub value: usize,
    pub list: Vec<&'static str>,
}
impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.onchange.eq(&other.onchange)
            && self.value.eq(&other.value)
            && self.list.iter().zip(other.list.iter()).all(|(x, y)| x == y)
    }
}

#[function_component(Dropdown)]
pub fn dropdown(props: &Props) -> Html {
    let list_ref = use_node_ref();
    let display_state = use_state(|| "display: none;");
    let show_state = use_state(|| None);
    let onclick = Callback::from({
        let list_ref = list_ref.clone();
        let display_state = display_state.clone();
        let show_state = show_state.clone();
        move |evt: MouseEvent| {
            evt.stop_propagation();
            display_state.set("display: block;");
            Timeout::new(0, {
                let show_state = show_state.clone();
                move || show_state.set(Some("show"))
            })
            .forget();

            let document = gloo::utils::document();
            EventListener::once(&document, "click", {
                let list = list_ref.cast::<HtmlElement>().unwrap();
                let display_state = display_state.clone();
                let show_state = show_state.clone();
                move |_| {
                    Timeout::new(500, move || {
                        show_state.set(None);
                        EventListener::once(&list, "transitionend", move |_| {
                            display_state.set("display: none;")
                        })
                        .forget();
                    })
                    .forget();
                }
            })
            .forget();
        }
    });

    let header_text: Option<&str> = props.list.get(props.value).map(|&v| v);
    let header_text = header_text.unwrap_or_default();
    let list = props
        .list
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let check = if i == props.value { "✓" } else { "　" };
            let onchange = props.onchange.clone();
            let onclick = Callback::from(move |_| onchange.emit(i));
            html!(<><div class="list-item" {onclick}><span>{check}</span>{v}</div></>)
        })
        .collect::<Vec<Html>>();

    let n = props.value + 1;
    let display = *display_state;
    let show = *show_state;

    let css = css! {r#"
        position: relative;
        border: 1px solid #666;
        border-radius: 8px;
        background: #101010;
        cursor: pointer;

        & .header {
            font-size: 16px;
            margin-left: 16px;
            height: 16px;
            display: flex;
            color: #ccc;

            & > span {
                display: block;
                font-size: 12px;
                margin: auto;
                margin-left: 12px;
                margin-right: 8px;
                top: 0;
                bottom: 0;
                color: #666;
            }
        }

        & .list {
            display: block;
            position: absolute;
            top: -1px;
            left: -1px;
            background: #ccc;
            border: 1px solid #ccc;
            border-radius: 8px;
            overflow: hidden;
            opacity: 0;
            filter: drop-shadow(0 0 0 rgba(0, 0, 0, 0));
            transition: all 0.5s;

            &.show {
                opacity: 1;
                filter: drop-shadow(0 20px 10px rgba(0, 0, 0, .7));
            }

            & .list-item {
                color: #222;
                display: block;
                cursor: pointer;
                min-width: 100%;
                padding-right: 16px;
                display: flex;
                white-space: nowrap;
                background: transparent;

                &:hover {
                    background: #30ffff66;
                }

                & > span {
                    display: block;
                    font-size: 12px;
                    margin: auto;
                    margin-left: 8;
                    margin-right: 12px;
                    top: 0;
                    bottom: 0;
                    color: #333;
                }
            }
        }
    "#};
    let dynamic_css = dynamic_css! {format!{r#"
        & .list {{
            {display}
        }}

        & .list .list-item:nth-of-type({n}) {{
            background: #30ffff;
        }}
    "#}};
    html! {
        <div class={classes!(css, dynamic_css)}>
            <div class="header" {onclick}>
                <div>{header_text}</div>
                <span>{"▼"}</span>
            </div>
            <div class={classes!("list", show)} ref={list_ref}>
                {list}
            </div>
        </div>
    }
}
