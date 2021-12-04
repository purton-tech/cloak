use horrorshow::owned_html;
use horrorshow::prelude::*;

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(page_title: impl Render, content: impl Render) -> impl Render {
    // owned_html _moves_ the arguments into the template. Useful for returning
    // owned (movable) templates.
    owned_html! {
        html {
            head {
                title : &page_title;
                meta(charset="utf-8") {}
                meta(http-equiv="X-UA-Compatible", content="IE=edge") {}
                meta(name="viewport", content="width=device-width, initial-scale=1") {}
                link(rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()) {}
            }
            body {
                div {
                    header {

                    }
                    aside {

                    }
                    main(class="container") {
                        section(class="content") {
                            :&content
                        }
                    }
                }
            }
        }
    }
}
