#![allow(non_snake_case)]
#![allow(unused_braces)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarType {
    Organisation,
    #[default]
    User,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl AvatarSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            AvatarSize::Small => "width='24' height='24'",
            AvatarSize::Medium => "width='24' height='24'",
            AvatarSize::Large => "width='24' height='24'",
            AvatarSize::ExtraLarge => "width='24' height='24'",
        }
    }
}

#[derive(Props)]
pub struct AvatarProps<'a> {
    avatar_size: Option<AvatarSize>,
    avatar_type: Option<AvatarType>,
    name: Option<&'a str>,
    _email: Option<&'a str>,
    _image_src: Option<&'a str>,
}

pub fn Avatar<'a>(cx: Scope<'a, AvatarProps<'a>>) -> Element {
    let avatar_size = if cx.props.avatar_size.is_some() {
        cx.props.avatar_size.unwrap()
    } else {
        Default::default()
    };
    let avatar_size = avatar_size.to_string();

    let mut the_name = "?".to_string();
    if let Some(name) = cx.props.name {
        the_name = if let Some(chr) = name.chars().next() {
            chr.to_string()
        } else {
            "?".to_string()
        };
    }

    let mut svg = format!(
        "<svg aria-hidden='true' xmlns='http://www.w3.org/2000/svg' {} viewBox='0 0 27 27'>
        <rect fill='rgb(125, 73, 193)' height='27' rx='12' width='27' x='0' y='0'></rect>
        <g fill='#fff' opacity='.5'>
            <circle cx='13.5' cy='30' r='13'></circle>
            <circle cx='13.5' cy='11' r='5'></circle>
        </g>
    </svg>",
        avatar_size
    );

    if let Some(avatar_type) = cx.props.avatar_type {
        if avatar_type == AvatarType::Organisation {
            svg = format!("<?xml version='1.0' encoding='UTF-8'?>
            <svg version='1.1' xmlns='http://www.w3.org/2000/svg' {} viewBox='0 0 50 50'>
                <rect width='100%' height='100%' fill='rgb(46, 77, 172)' />
                <text fill='#fff' font-size='26' font-weight='500' x='50%' y='55%' dominant-baseline='middle' text-anchor='middle'>
                    {}
                </text>
            </svg>", avatar_size, the_name);
        } else {
            svg = format!("<svg version='1.1' xmlns='http://www.w3.org/2000/svg' {} viewBox='0 0 50 50'>
                <rect width='100%' height='100%' fill='rgb(46, 77, 172)' />
                <text fill='#fff' font-size='26' font-weight='500' x='50%' y='55%' dominant-baseline='middle' text-anchor='middle'>
                    {}
                </text>
            </svg>", avatar_size, the_name);
        }
    }

    cx.render(rsx!({ LazyNodes::new(|f| f.text(format_args!("{svg}"))) }))
}
