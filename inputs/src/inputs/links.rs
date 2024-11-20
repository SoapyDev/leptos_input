use icondata::Icon;
use leptos::{component, view, IntoView, MaybeSignal, Show};
use leptos_icons::Icon;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkStyle {
    Underline,
    Outline,
    Rounded,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkColor {
    Primary,
    Secondary,
    Error,
    Success,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkAnimation {
    Fill,
    Push,
    Float,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkTarget {
    _Blank,
    _Self,
    _Parent,
    _Top,
}
#[component]
pub fn Link(
    #[prop(default = LinkStyle::Text)] style: LinkStyle,
    #[prop(default = LinkColor::Primary)] color: LinkColor,
    #[prop(default = LinkSize::Medium)] size: LinkSize,
    #[prop(default = LinkAnimation::None)] animation: LinkAnimation,
    #[prop(default = LinkTarget::_Blank)] target: LinkTarget,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] text: MaybeSignal<String>,
    #[prop(default = None)] icon: Option<Icon>,
) -> impl IntoView {
    let target = match target {
        LinkTarget::_Blank => "_blank",
        LinkTarget::_Self => "_self",
        LinkTarget::_Parent => "_parent",
        LinkTarget::_Top => "_top",
    };

    let rel = if target == "_blank" {
        "noopener noreferrer"
    } else {
        "none"
    };

    view! {
        <a
            class="link"
            class:link-underline=style == LinkStyle::Underline
            class:link-outline=style == LinkStyle::Outline
            class:link-rounded=style == LinkStyle::Rounded
            class:link-text=style == LinkStyle::Text
            class:primary=color == LinkColor::Primary
            class:secondary=color == LinkColor::Secondary
            class:error=color == LinkColor::Error
            class:success=color == LinkColor::Success
            class:text=color == LinkColor::Text
            class:small=size == LinkSize::Small
            class:medium=size == LinkSize::Medium
            class:large=size == LinkSize::Large
            class:fill=animation == LinkAnimation::Fill
            class:push=animation == LinkAnimation::Push
            class:float=animation == LinkAnimation::Float
            target=target
            rel=rel
            href=href
        >
            <span>
                <Show when=move || icon.is_some()>
                    <Icon icon=icon.unwrap() width="1.5rem" height="1.5rem" />
                </Show>
                {text}
            </span>
        </a>
    }
}
