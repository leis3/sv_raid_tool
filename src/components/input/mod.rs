mod name;
mod star;
mod type_;
mod search;

use name::NameInput;
use star::StarInput;
// use type_::TypeInput;
use search::SearchInput;

use yew::prelude::*;
use yew::{html, Html, Properties};

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct InputProps {
    pub on_search: Callback<Input>
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub name: String,
    pub star: u8,
    pub type_: String,
    name_cb: Callback<String>,
    star_cb: Callback<u8>,
    _type_cb: Callback<String>,
    search_cb: Callback<()>
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
        let _type_cb = {
            let link = ctx.link().clone();
            Callback::from(move |e: String| {
                link.send_message(InputMsg::Type(e));
            })
        };
        let search_cb = {
            let _props = ctx.props().on_search.clone();
            let link = ctx.link().clone();
            Callback::from(move |_: ()| {
                link.send_message(InputMsg::Search);
            })
        };
        Self {
            name_cb, star_cb, _type_cb, search_cb,
            star: 6,
            ..Default::default()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="background-color: rgba(0,0,255,.1);">
                <StarInput on_input={self.star_cb.clone()} />
                <NameInput on_input={self.name_cb.clone()} star={self.star} />
                // <TypeInput on_input={self._type_cb.clone()} />
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
