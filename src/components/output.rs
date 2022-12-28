use yew::prelude::*;
use yew::{html, Html, Properties};
use sv_raid::Raid;
use std::rc::Rc;
use itertools::Itertools;
use std::cmp::Reverse;

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct OutputProps {
    pub result: Rc<Option<Raid>>
}

#[derive(Debug, Clone, Default)]
pub struct Output {
    pub data: Option<Raid>
}

impl Component for Output {
    type Message = ();
    type Properties = OutputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(data) = &self.data {
            html! {
                <div class="container" style="width:50%;">
                    
                    <h2>{&data.name}</h2>

                    <table class="table table-bordered caption-top">
                        <caption>{"とくせい"}</caption>
                        <thead>
                            <tr>
                                <th scope="col" class="text-center">{"名前"}</th>
                                <th scope="col" class="text-center">{"説明"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {data.abilities.iter().map(|a| html! {
                                <tr>
                                    <td class="text-center">{&a.name}</td>
                                    <td class="text-center">{&a.description}</td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>

                    <table class="table table-bordered caption-top">
                        <caption>{"ステータス"}</caption>
                        <thead>
                            <tr>
                            <th scope="col" class="text-center">{"HP"}</th>
                            <th scope="col" class="text-center">{"こうげき"}</th>
                            <th scope="col" class="text-center">{"とくこう"}</th>
                            <th scope="col" class="text-center">{"ぼうぎょ"}</th>
                            <th scope="col" class="text-center">{"とくぼう"}</th>
                            <th scope="col" class="text-center">{"すばやさ"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="text-center">{data.hp}</td>
                                <td class="text-center">{data.atk}</td>
                                <td class="text-center">{data.sp_atk}</td>
                                <td class="text-center">{data.def}</td>
                                <td class="text-center">{data.sp_def}</td>
                                <td class="text-center">{data.speed}</td>
                            </tr>
                        </tbody>
                    </table>
                    
                    <table class="table table-bordered caption-top">
                        <caption>{"全体行動"}</caption>
                        <tbody>
                            {data.actions.iter().map(|s| html! {
                                <tr>
                                    <td>{&s}</td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>

                    <table class="table table-bordered caption-top">
                        <caption>{"わざ"}</caption>
                        <thead>
                        <tr>
                            <th scope="col">{"名前"}</th>
                            <th scope="col">{"タイプ"}</th>
                            <th scope="col">{"種類"}</th>
                            <th scope="col">{"威力"}</th>
                            <th scope="col">{"説明"}</th>
                        </tr>
                        </thead>
                        <tbody>
                            {data.moves.iter().sorted_unstable_by_key(|m| {
                                // カテゴリ, タイプ, 威力(降順)の順にソート
                                (m.category.clone() as i32, m.r#type.clone() as i32, Reverse(m.power))
                            }).map(|m| html! {
                                <tr>
                                    <td class="text-center">{&m.name}</td>
                                    <td class="text-center">{format!("{}", m.r#type)}</td>
                                    <td class="text-center">{format!("{}", m.category)}</td>
                                    <td class="text-center">{m.power}</td>
                                    <td>{&m.description}</td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>
            }
        }
        else {
            html! {}
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let data = &*ctx.props().result;
        self.data = data.clone();
        true
    }
}