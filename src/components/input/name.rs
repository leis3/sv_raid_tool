use yew::prelude::*;
use yew::virtual_dom::{VNode, Listener};
use yew::{html, Html, Properties};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlLiElement, CompositionEvent, Event,};
use std::rc::Rc;
use once_cell::sync::Lazy;

static POKEMON_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    let data = include_str!("../../../data/raid_pokemon_list.txt");
    data.split("\n").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
});


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NameProps {
    pub on_input: Callback<String>
}

#[derive(Debug, Clone, Default)]
pub struct NameInput {
    pub name: String,
    filter: String
}

#[derive(Debug, Clone)]
pub enum NameMsg {
    Input(String),
    Select(String),
    CompUpdate(String),
    CompEnd
}

#[derive(Debug, Clone, Default)]
struct CompositionUpdateListener {
    pub cb: Callback<String>
}

#[derive(Debug, Clone, Default)]
struct CompositionEndListener {
    pub cb: Callback<String>
}

impl Listener for CompositionUpdateListener {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        yew::virtual_dom::ListenerKind::other("compositionupdate".into())
    }

    fn handle(&self, event: Event) {
        let value = event
            .dyn_into::<CompositionEvent>().unwrap()
            .data().unwrap();
        if wana_kana::is_kana::is_kana(&value) {
            self.cb.emit(value);
        }
    }

    fn passive(&self) -> bool {
        false
    }
}

impl Listener for CompositionEndListener {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        yew::virtual_dom::ListenerKind::other("compositionend".into())
    }

    fn handle(&self, _event: Event) {
        let data = _event
            .target().unwrap()
            .dyn_into::<HtmlInputElement>().unwrap()
            .value();
        self.cb.emit(data);
    }

    fn passive(&self) -> bool {
        false
    }
}


impl Component for NameInput {
    type Message = NameMsg;
    type Properties = NameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = {
            let link = ctx.link().clone();
            let props = ctx.props().on_input.clone();
            Callback::from(move |e: Event| {
                let data = e.target().unwrap()
                    .dyn_into::<HtmlInputElement>().unwrap()
                    .value();
                props.emit(data.clone());
                link.send_message(NameMsg::Input(data));
            })
        };

        let on_input = {
            let link = ctx.link().clone();
            let props = ctx.props().on_input.clone();
            Callback::from(move |e: InputEvent| {
                match e.input_type().as_str() {
                    "deleteContentForward" | "deleteContentBackward" => {
                        let data = e.target().unwrap()
                            .dyn_into::<HtmlInputElement>().unwrap()
                            .value();
                        props.emit(data.clone());
                        link.send_message(NameMsg::Input(data));
                    },
                    _ => {}
                }
            })
        };

        let on_click = {
            let link = ctx.link().clone();
            let props = ctx.props().on_input.clone();
            Callback::from(move |e: MouseEvent| {
                let data = e.target().unwrap()
                    .dyn_into::<HtmlLiElement>().unwrap()
                    .inner_text();
                log::info!("send_message: {data:?}");
                props.emit(data.clone());
                link.send_message(NameMsg::Select(data));
            })
        };

        let on_comp_update = {
            let link = ctx.link().clone();
            Callback::from(move |e: String| {
                link.send_message(NameMsg::CompUpdate(e));
            })
        };

        let on_comp_end = {
            let link = ctx.link().clone();
            Callback::from(move |_: String| {
                link.send_message(NameMsg::CompEnd)
            })
        };

        let mut input = html! {
            <input type="text" onchange={on_change} oninput={on_input} list="pklist" class="form-control" data-bs-toggle="dropdown"
                value={self.name.clone()} name="name" id="name" autocomplete="off" />
        };
        if let VNode::VTag(tag) = &mut input {
            tag.add_listener(Rc::new(CompositionUpdateListener {cb: on_comp_update}));
            tag.add_listener(Rc::new(CompositionEndListener {cb: on_comp_end}));
        } else { unreachable!() }
        html! {
            <div style="width: 20em; padding: 10px 20px;">
                <label class="form-label" for="name">{"名前"}</label>
                <div>
                    {input}
                    <div class="dropdown-menu overflow-auto" aria-labelledby="name" style="max-height:20rem;">
                        <ul class="text-center p-0">
                            {POKEMON_LIST.iter().filter(|s| s.contains(&format!("{}{}", self.name, self.filter))).map(|name| html! {
                                <li onclick={on_click.clone()} class="dropdown-item text-start">{name.clone()}</li>
                            }).collect::<Html>()}
                        </ul>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NameMsg::Input(s) => {
                self.name = s;
                self.filter.clear();
                true
            },
            NameMsg::Select(s) => {
                log::info!("Namemsg::Select({s:?})");
                self.name = s;
                self.filter.clear();
                true
            },
            NameMsg::CompUpdate(s) => {
                // sには現在編集のテキスト全体が入る
                // ここで再レンダリングするとIMEの編集セッションが維持されないのでfalseを返す
                self.filter = s;
                false
            },
            NameMsg::CompEnd => {
                self.filter.clear();
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }
}