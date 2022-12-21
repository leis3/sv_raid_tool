use yew::prelude::*;
use yew::{html, Html, Properties};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use crate::res::{Type, type_color, type_list};



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

    fn create(_ctx: &Context<Self>) -> Self {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.type_ = msg.clone();
        true
    }
}