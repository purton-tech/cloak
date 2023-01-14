#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelativeTimeFormat {
    Datetime,
    Relative,
    Duration,
    Auto,
    Micro,
    Elapsed,
}

impl RelativeTimeFormat {
    pub fn to_string(&self) -> &'static str {
        match self {
            RelativeTimeFormat::Datetime => "datetime",
            RelativeTimeFormat::Relative => "relative",
            RelativeTimeFormat::Duration => "duration",
            RelativeTimeFormat::Auto => "auto",
            RelativeTimeFormat::Micro => "micro",
            RelativeTimeFormat::Elapsed => "elapsed",
        }
    }
}

impl Default for RelativeTimeFormat {
    fn default() -> Self {
        RelativeTimeFormat::Relative
    }
}

#[derive(Props)]
pub struct RelativeTimeProps<'a> {
    format: Option<RelativeTimeFormat>,
    datetime: &'a str,
}

pub fn RelativeTime<'a>(cx: Scope<'a, RelativeTimeProps<'a>>) -> Element {
    let format = if cx.props.format.is_some() {
        cx.props.format.unwrap()
    } else {
        Default::default()
    };

    cx.render(rsx!({
        LazyNodes::new(|f| {
            f.text(format_args!(
                "<relative-time format='{}' datetime='{}'></relative-time>",
                format.to_string(),
                cx.props.datetime
            ))
        })
    }))
}
