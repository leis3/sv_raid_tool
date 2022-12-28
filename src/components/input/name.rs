use yew::prelude::*;
use yew::virtual_dom::{VNode, Listener};
use yew::{html, Html, Properties};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlLiElement, Event,};
use std::rc::Rc;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use itertools::Itertools;

static POKEMON_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    let data = include_str!("../../../data/raid_pokemon_list.txt");
    data.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .sorted_unstable()
        .collect()
});


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NameProps {
    pub on_input: Callback<String>,
    pub star: u8
}

#[derive(Debug, Clone, Default)]
pub struct NameInput {
    pub name: String
}

#[derive(Debug, Clone, Default)]
struct CompositionEndListener {
    pub cb: Callback<String>
}

impl Listener for CompositionEndListener {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        yew::virtual_dom::ListenerKind::other("compositionend".into())
    }

    fn handle(&self, event: Event) {
        let data = event
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
    type Message = String;
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
                link.send_message(data);
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
                        link.send_message(data);
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
                props.emit(data.clone());
                link.send_message(data);
            })
        };

        let on_comp_end = {
            let link = ctx.link().clone();
            Callback::from(move |e: String| {
                link.send_message(e);
            })
        };

        let on_reset = {
            let link = ctx.link().clone();
            let props = ctx.props().on_input.clone();
            Callback::from(move |_| {
                props.emit(String::new());
                link.send_message(String::new());
            })
        };

        // 名前がname_filterで部分一致するものでフィルタリング
        let name_filter = wana_kana::to_katakana::to_katakana(&self.name);

        // 選択された難易度に出現するポケモンでフィルタリング
        let star_filter = sv_raid::raid_filter(None, Some(ctx.props().star))
            .into_iter()
            .map(|r| r.name)
            .collect::<HashSet<_>>();

        let filtered = POKEMON_LIST
            .iter()
            .filter(|s| s.contains(&name_filter))
            .cloned()
            .collect::<HashSet<String>>();
            
        let pk_list = filtered
            .intersection(&star_filter)
            .sorted_unstable()
            .cloned()
            .collect_vec();

        let mut input = html! {
            <input type="text" onchange={on_change} oninput={on_input} class="form-control" data-bs-toggle="dropdown"
                value={self.name.clone()} name="name" id="name" autocomplete="off" />
        };
        if let VNode::VTag(tag) = &mut input {
            tag.add_listener(Rc::new(CompositionEndListener {cb: on_comp_end}));
        } else { unreachable!() }

        html! {
            <div style="width: 20em; padding: 10px 20px;">
                <label class="form-label" for="name">{"名前"}</label>
                <div>
                    <div class="input-group">
                        {input}
                        <button type="button" class="btn btn-light border-secondary rounded-end" onclick={on_reset}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-x-lg" viewBox="0 0 16 16">
                                <path d="M2.146 2.854a.5.5 0 1 1 .708-.708L8 7.293l5.146-5.147a.5.5 0 0 1 .708.708L8.707 8l5.147 5.146a.5.5 0 0 1-.708.708L8 8.707l-5.146 5.147a.5.5 0 0 1-.708-.708L7.293 8 2.146 2.854Z"/>
                            </svg>
                        </button>
                        <div class="dropdown-menu overflow-auto" aria-labelledby="name" style="max-height:20rem;">
                            <ul class="text-center p-0">
                                {pk_list.iter().map(|name| html! {
                                    <li onclick={on_click.clone()} class="dropdown-item text-start">{name.clone()}</li>
                                }).collect::<Html>()}
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.name = msg;
        true
    }
}