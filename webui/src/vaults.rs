use axum::response::Html;
use horrorshow::owned_html;
use horrorshow::prelude::*;

fn home_content() -> impl Render {
    owned_html! {
        h1 { :"Home Page" }
    }
}

pub async fn index() -> Html<String> {
    let page = crate::layout::layout("Home", home_content());
    Html(page.into_string().unwrap())
}
