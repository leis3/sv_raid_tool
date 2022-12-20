use web_sys::{MouseEvent, InputEvent};
use yew::{html, Html, function_component, Callback, use_state, Properties};
use yew::prelude::*;


#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct Input {
    pub name: String,
    pub star: u8,
    pub ty: String
}


/*

指定するもの

・ポケモンの名前
入力欄があり、入力を始めると部分一致するポケモンのリストを入力欄の下に表示し、それらをクリックすると自動でその名前が入力される。

・難易度
1～6(7)を選択

・テラスタイプ
hyperWikiのパーティ編成ツールのようにボタンで表示したい

*/

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct TypeProps {
    pub name: String,
    pub id: u8,
    pub ty: UseStateHandle<String>
}

#[function_component(TypeItem)]
pub fn type_item(props: &TypeProps) -> Html {
    let on_input = {
        let props = props.clone();
        let ty = props.ty.clone();
        Callback::from(move |_: InputEvent| {
            ty.set(format!("{}", props.name));
        })
    };

    html! {
        <div class="col" style="margin: 5px;">
            <input type="radio" oninput={on_input} class="btn-check" name="ty" value={format!("{}", props.name)} id={format!("ty{}", props.id)} autocomplete="off"/>
            <label class="btn btn-outline-primary" for={format!("ty{}", props.id)} style="width: 80%;">{&props.name}</label>
        </div>
    }
}

#[function_component(ToolInput)]
pub fn tool_input() -> Html {
    let name = use_state(|| String::default());
    let star = use_state(|| 6u8);
    let ty = use_state(|| String::default());

    let on_search_click = {
        let name = name.clone();
        let star = star.clone();
        let ty = ty.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let data = sv_raid::raid(&*name, *star);
            log::info!("{:?}", data);
            log::info!("ty: {}", &*ty);
        })
    };

    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            name.set(e.data().unwrap_or_default());
        })
    };

    let on_star_input = {
        let star = star.clone();
        Callback::from(move |e: InputEvent| {
            log::info!("{:?}", e.data());
            star.set(e.data().map(|s| s.parse().unwrap()).unwrap_or(6));
        })
    };

    let types = vec!["ノーマル", "かくとう", "ひこう", "どく", "じめん", "いわ", "むし", "ゴースト", "はがね", "ほのお", "みず", "くさ", "でんき", "エスパー", "こおり", "ドラゴン", "あく", "フェアリー"];

    html! {
        <div style="background-color: rgba(0,0,255,.1);">
            <div style="width: 20em; padding: 10px 20px;">
                <label class="form-label" for="name">{"名前"}</label>
                <input type="text" class="form-control" oninput={on_name_input} name="name" id="name" />
            </div>
            <div style="width: 10em; padding: 10px 20px">
                <label class="form-label">{"難易度"}</label>
                <select class="form-select" oninput={on_star_input} id="stars" name="stars">
                    <option value="1">{1}</option>
                    <option value="2">{2}</option>
                    <option value="3">{3}</option>
                    <option value="4">{4}</option>
                    <option value="5">{5}</option>
                    <option value="6" selected=true>{6}</option>
                </select>
            </div>
            <div style="padding: 20px; width: 70%;">
                <div>{"テラスタイプ"}</div>
                <div class="row row-cols-6">
                    // TODO: 各ボタンの背景色と押されたときの動作といずれか1つのみ選択できるようにする
                    // 各選択肢はテキストじゃなくてタイプアイコンでも
                    {
                        types.iter().enumerate().map(|(i, name)| html! {
                            <TypeItem name={name.clone()} id={i as u8} ty={ty.clone()} />
                        }).collect::<Html>()
                    }
                </div>
            </div>
            <div style="padding: 10px 20px;">
                <button type="button" class="btn btn-info" onclick={on_search_click}>{"Search"}</button>
            </div>
        </div>
    }
}