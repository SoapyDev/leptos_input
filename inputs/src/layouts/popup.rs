use crate::{Button, ButtonSize, ButtonStyle, DisplayStrategy, Line};
use icondata::IoClose;
use leptos::html::Dialog;
use leptos::{
    component, create_node_ref, view, Children, IntoView, MaybeSignal, RwSignal, SignalSet,
};
use leptos_icons::Icon;
use leptos_use::on_click_outside;

#[derive(Copy, Clone, PartialEq)]
pub enum PopupSize {
    Content,
    Small,
    Medium,
    Large,
    Full,
}
#[derive(Copy, Clone, PartialEq)]
pub enum PopupPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    Center,
    Relative,
}

#[component]
pub fn Popup(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] visible: RwSignal<bool>,
    #[prop(default = PopupSize::Medium)] size: PopupSize,
    #[prop(default = PopupPosition::Center)] position: PopupPosition,
    #[prop(optional)] on_close: Option<fn()>,
    #[prop(optional)] on_outside_click: Option<fn()>,
    children: Children,
) -> impl IntoView {
    let target = create_node_ref::<Dialog>();
    let _ = on_click_outside(target, move |_| {
        if let Some(on_outside_click) = on_outside_click {
            on_outside_click();
        }
    });

    view! {
        <dialog
            open=visible
            node_ref=target
            class="popup"
            class:popup-content= size == PopupSize::Content
            class:popup-small= size == PopupSize::Small
            class:popup-medium= size == PopupSize::Medium
            class:popup-large= size == PopupSize::Large
            class:popup-full= size == PopupSize::Full
            class:popup-top-left= position == PopupPosition::TopLeft
            class:popup-top-center= position == PopupPosition::TopCenter
            class:popup-top-right= position == PopupPosition::TopRight
            class:popup-bottom-left= position == PopupPosition::BottomLeft
            class:popup-bottom-center= position == PopupPosition::BottomCenter
            class:popup-bottom-right= position == PopupPosition::BottomRight
            class:popup-center-left= position == PopupPosition::CenterLeft
            class:popup-center-right= position == PopupPosition::CenterRight
            class:popup-center= position == PopupPosition::Center
            class:popup-relative= position == PopupPosition::Relative
        >
            <Line justify=DisplayStrategy::End>
                <Button
                    style=ButtonStyle::Text
                    size=ButtonSize::Small
                    text="Close"
                    content_after=view!{<Icon icon=IoClose class="close-icon"/>}
                    on:click=move |_| if let Some(on_close) = on_close { on_close() }else{ visible.set(false) }
                />
            </Line>
            <Line>
                <h2>{title}</h2>
            </Line>
            {children()}
        </dialog>
    }
}
