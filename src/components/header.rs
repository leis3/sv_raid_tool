use yew::{html, Html, function_component};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a class="navbar-brand" href="#">{"テラレイドバトル用ツール"}</a>
            </div>
        </nav>
    }
}