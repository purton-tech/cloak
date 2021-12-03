use horrorshow::owned_html;
use horrorshow::prelude::*;

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(page_title: impl Render, content: impl Render) -> impl Render {
    // owned_html _moves_ the arguments into the template. Useful for returning
    // owned (movable) templates.
    owned_html! {
        head {
            title : &page_title;
            meta(charset="utf-8") {}
        }
        body {
            :&content
        }
    }
}
