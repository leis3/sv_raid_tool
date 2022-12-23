use yew::prelude::*;
use yew::{html, Html, Properties};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SearchProps {
    pub on_click: Callback<()>
}

#[derive(Debug, Clone, Default)]
pub struct SearchInput;

impl Component for SearchInput {
    type Message = ();
    type Properties = SearchProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().on_click.clone();
        let on_click = {
            Callback::from(move |_e: MouseEvent| {
                props.emit(());
            })
        };

        html! {
            <div style="padding: 20px 20px;">
                <button type="button" class="btn btn-info px-4" onclick={on_click}>
                {"検索"}
                </button>
            </div>
        }
    }
}