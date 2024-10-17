use leptos::{component, view, IntoView, MaybeSignal};
#[derive(PartialEq, Clone, Copy)]
pub enum ButtonStyle {
    Outline,
    Solid, 
    RoundedOutline,
    RoundedSolid
}

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Error,
    Success,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    FullSize
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonAnimation {
    Fill, 
    Push,
    Float
}
#[component]
pub fn Button(
    #[prop(into, default = MaybeSignal::from(String::from("Click me")))] text: MaybeSignal<String>,
    #[prop(default = ButtonStyle::Outline)] style: ButtonStyle,
    #[prop(default = ButtonColor::Primary)] color: ButtonColor,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
    #[prop(default = ButtonAnimation::Push)] animation: ButtonAnimation
) -> impl IntoView {
    
    
    view! {
        <button class="button"
            class:outline=style == ButtonStyle::Outline
            class:solid=style == ButtonStyle::Solid
            class:rounded-outline=style == ButtonStyle::RoundedOutline
            class:rounded-solid=style == ButtonStyle::RoundedSolid
            class:primary=color == ButtonColor::Primary
            class:secondary=color == ButtonColor::Secondary
            class:error=color == ButtonColor::Error
            class:success=color == ButtonColor::Success
            class:small=size == ButtonSize::Small
            class:medium=size == ButtonSize::Medium
            class:large=size == ButtonSize::Large
            class:full-size=size == ButtonSize::FullSize
            class:fill=animation == ButtonAnimation::Fill
            class:push=animation == ButtonAnimation::Push
            class:float=animation == ButtonAnimation::Float
        >
            {text}
        </button>
    }
}