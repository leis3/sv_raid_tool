use yew::prelude::*;
use yew::{html, Html, Properties};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;


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

    fn create(_ctx: &Context<Self>) -> Self {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.star = msg;
        true
    }
}