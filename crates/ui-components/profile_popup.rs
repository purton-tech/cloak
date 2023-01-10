use assets::files::{avatar_svg, button_select_svg};
use db::queries::users::User;
use dioxus::prelude::*;
use primer_rsx::*;

struct ProfilePopupProps {
    user_name_or_email: String,
}

pub fn profile_popup(user: User) -> String {
    fn app(cx: Scope<ProfilePopupProps>) -> Element {
        cx.render(rsx! {
            {
                LazyNodes::new(|f| f.text(format_args!("<turbo-frame class='width-full' id='profile-popup'>")))
            }
            DropDown {
                direction: Direction::NorthEast,
                button_text: &cx.props.user_name_or_email,
                prefix_image_src: avatar_svg.name,
                suffix_image_src: button_select_svg.name,
                class: "width-full",
                DropDownLink {
                    href: "profile",
                    target: "_top",
                    "Profile"
                }
                DropDownLink {
                    href: "#",
                    target: "_top",
                    drawer_trigger: "logout-drawer",
                    "Log Out"
                }
            }
            {
                LazyNodes::new(|f| f.text(format_args!("</turbo-frame>")))
            }
        })
    }

    let name = if user.first_name.is_some() && user.last_name.is_some() {
        format!("{} {}", user.first_name.unwrap(), user.last_name.unwrap())
    } else {
        user.email
    };

    let mut app = VirtualDom::new_with_props(
        app,
        ProfilePopupProps {
            user_name_or_email: name,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
