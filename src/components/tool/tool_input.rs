use yew::prelude::*;
use yew::virtual_dom::{VNode, Listener};
use yew::{html, Html, Properties};
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement, HtmlLiElement, CompositionEvent};
use crate::res::{Type, type_color, type_list};
use once_cell::sync::Lazy;
use std::rc::Rc;

static POKEMON_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    let data = include_str!("../../../data/raid_pokemon_list.txt");
    data.split("\n").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
});

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct InputProps {
    pub on_search: Callback<Input>
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub name: String,
    pub star: u8,
    pub type_: String,
    pub name_cb: Callback<String>,
    pub star_cb: Callback<u8>,
    pub type_cb: Callback<String>,
    pub search_cb: Callback<()>
}

#[derive(Debug, Clone)]
pub enum InputMsg {
    Name(String),
    Star(u8),
    Type(String),
    Search
}


impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let name_cb = {
            let link = ctx.link().clone();
            Callback::from(move |e: String| {
                link.send_message(InputMsg::Name(e));
            })
        };
        let star_cb = {
            let link = ctx.link().clone();
            Callback::from(move |e: u8| {
                link.send_message(InputMsg::Star(e));
            })
        };
        let type_cb = {
            let link = ctx.link().clone();
            Callback::from(move |e: String| {
                link.send_message(InputMsg::Type(e));
            })
        };
        let search_cb = {
            let props = ctx.props().on_search.clone();
            let link = ctx.link().clone();
            Callback::from(move |e: ()| {
                link.send_message(InputMsg::Search);
            })
        };
        Self {
            name_cb, star_cb, type_cb, search_cb,
            star: 6,
            ..Default::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="background-color: rgba(0,0,255,.1);">
                <NameInput on_input={self.name_cb.clone()} />
                <StarInput on_input={self.star_cb.clone()} />
                <TypeInput on_input={self.type_cb.clone()} />
                <SearchInput on_click={self.search_cb.clone()} />
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMsg::Name(name) => {
                self.name = name;
            },
            InputMsg::Star(star) => {
                self.star = star;
            },
            InputMsg::Type(type_) => {
                self.type_ = type_;
            },
            InputMsg::Search => {
                ctx.props().on_search.emit(self.clone());
            }
        }
        true
    }
}

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

impl Listener for CompositionUpdateListener {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        yew::virtual_dom::ListenerKind::other("compositionupdate".into())
    }

    fn handle(&self, event: web_sys::Event) {
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

#[derive(Debug, Clone, Default)]
struct CompositionEndListener {
    pub cb: Callback<()>
}

impl Listener for CompositionEndListener {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        yew::virtual_dom::ListenerKind::other("compositionend".into())
    }

    fn handle(&self, event: web_sys::Event) {
        self.cb.emit(());
    }

    fn passive(&self) -> bool {
        false
    }
}

impl Component for NameInput {
    type Message = NameMsg;
    type Properties = NameProps;

    fn create(ctx: &Context<Self>) -> Self {
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
                link.send_message(NameMsg::Input(data.clone()));
            })
        };

        let on_click = {
            let link = ctx.link().clone();
            Callback::from(move |e: MouseEvent| {
                let data = e.target().unwrap()
                    .dyn_into::<HtmlLiElement>().unwrap()
                    .inner_text();
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
            Callback::from(move |_: ()| {
                link.send_message(NameMsg::CompEnd)
            })
        };

        let mut input = html! {
            <input type="text" onchange={on_change} list="pklist" class="form-control" data-bs-toggle="dropdown"
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
                            // ToDo: Define new <PokemonListItem> as Component in order to rerender only this element
                            {POKEMON_LIST.iter().filter(|s| s.contains(&format!("{}{}", self.name, self.filter))).map(|name| html! {
                                <li onclick={on_click.clone()} class="dropdown-item text-start">{name.clone()}</li>
                            }).collect::<Html>()}
                        </ul>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NameMsg::Input(s) => {
                self.name = s;
                true
            },
            NameMsg::Select(s) => {
                self.name = s;
                true
            },
            NameMsg::CompUpdate(s) => {
                self.filter.push_str(&s);
                false
            },
            NameMsg::CompEnd => {
                let t = self.filter.clone();
                self.filter.clear();
                self.name.push_str(&t);
                false
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StarProps {
    pub on_input: Callback<u8>
}

#[derive(Debug, Clone, Default)]
pub struct StarInput {
    pub star: u8
}

impl Component for StarInput {
    type Message = u8;
    type Properties = StarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { star: 6 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let props = ctx.props().on_input.clone();
        let onchange = {
            Callback::from(move |e: Event| {
                let value = e.target().unwrap()
                    .dyn_into::<HtmlSelectElement>().unwrap()
                    .value()
                    .parse::<u8>().unwrap();
                props.emit(value);
                link.send_message(value);
            })
        };

        html! {
            <div style="width: 10em; padding: 10px 20px">
                <label class="form-label">{"難易度"}</label>
                <select class="form-select" onchange={onchange} id="stars" name="stars">
                    <option value="6" selected=true>{6}</option>
                    <option value="5">{5}</option>
                    <option value="4">{4}</option>
                    <option value="3">{3}</option>
                    <option value="2">{2}</option>
                    <option value="1">{1}</option>
                </select>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.star = msg;
        true
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TypeProps {
    pub on_input: Callback<String>
}

#[derive(Debug, Clone, Default)]
pub struct TypeInput {
    pub type_: String
}

impl Component for TypeInput {
    type Message = String;
    type Properties = TypeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input = {
            let link = ctx.link().clone();
            let props = ctx.props().on_input.clone();
            Callback::from(move |e: InputEvent| {
                let data = e.target().unwrap()
                    .dyn_into::<HtmlInputElement>().unwrap()
                    .value();
                props.emit(data.clone());
                link.send_message(data.clone());
            })
        };


        html! {
            <div style="padding: 20px; width: 70%;">
                <div>{"テラスタイプ"}</div>
                <div class="row row-cols-6">
                    {for type_list().iter().enumerate().map(|(i, name)| {
                        let type_ = Type::from(name.as_str());
                        let color = type_color(type_);
                        html! {
                            <div class="col" style="margin: 5px;">
                                <input type="radio" oninput={on_input.clone()} class="btn-check" name="ty" value={format!("{name}")} id={format!("ty{i}")} />
                                <label class="btn btn-outline-primary" for={format!("ty{i}")} style={format!("width: 80%; background-color:rgba{color:?};")}>{name}</label>
                            </div>
                        }}
                    )}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.type_ = msg.clone();
        true
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SearchProps {
    pub on_click: Callback<()>
}

#[derive(Debug, Clone, Default)]
pub struct SearchInput;

impl Component for SearchInput {
    type Message = ();
    type Properties = SearchProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().on_click.clone();
        let on_click = {
            Callback::from(move |e: MouseEvent| {
                props.emit(());
            })
        };

        html! {
            <div style="padding: 10px 20px;">
                <button type="button" class="btn btn-info" onclick={on_click}>{"Search"}</button>
            </div>
        }
    }
}