use crate::{DisplayStrategy, Line};
use leptos::{component, view, Children, ChildrenFn, IntoView, MaybeSignal, View};

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonStyle {
    Outline,
    Solid,
    Text,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonRoundness {
    None,
    Rounded,
    Circle,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Error,
    Success,
    None,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    FullSize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonAnimation {
    Fill,
    Push,
    Float,
}
#[component]
pub fn Button(
    #[prop(into, default = MaybeSignal::from(String::from("Submit")))] text: MaybeSignal<String>,
    #[prop(default = ButtonStyle::Outline)] style: ButtonStyle,
    #[prop(default = ButtonRoundness::Rounded)] roundness: ButtonRoundness,
    #[prop(default = ButtonColor::Primary)] color: ButtonColor,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = ButtonAnimation::Push)] animation: ButtonAnimation,
    #[prop(into, default=None)] content_before: Option<View>,
    #[prop(into, default=None)] content_after: Option<View>,
) -> impl IntoView {
    view! {
        <button class="button"
            class:outline=style == ButtonStyle::Outline
            class:solid=style == ButtonStyle::Solid
            class:text=style == ButtonStyle::Text
            class:rounded = roundness == ButtonRoundness::Rounded
            class:circular = roundness == ButtonRoundness::Circle
            class:primary=color == ButtonColor::Primary
            class:secondary=color == ButtonColor::Secondary
            class:error=color == ButtonColor::Error
            class:success=color == ButtonColor::Success
            class:none=color == ButtonColor::None
            class:small=size == ButtonSize::Small
            class:medium=size == ButtonSize::Medium
            class:large=size == ButtonSize::Large
            class:full=size == ButtonSize::FullSize
            class:fill=animation == ButtonAnimation::Fill
            class:push=animation == ButtonAnimation::Push
            class:float=animation == ButtonAnimation::Float
        >
            <Line justify=DisplayStrategy::Center align=DisplayStrategy::Center>
                {content_before}
                {text}
                {content_after}
            </Line>
        </button>
    }
}
