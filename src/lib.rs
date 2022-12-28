mod components;
mod res;

use components::{Header, Input, Output};

use sv_raid::Raid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::rc::Rc;


#[derive(Debug, Default)]
struct App {
    pub data: Rc<Option<Raid>>
}

impl Component for App {
    type Message = Option<Raid>;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.data = Rc::new(msg);
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_search = {
            let link = ctx.link().clone();
            Callback::from(move |e: Input| {
                let result = sv_raid::raid(&e.name, e.star);
                link.send_message(result);
            })
        };

        html! {
            <>
                <Header />
                <Input on_search={on_search} />
                <Output result={self.data.clone()} />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}