use crate::inputs::text_inputs::InputText;
use inputs::{text_inputs::text::InputTextType, InputTextState};
use leptos::*;

mod inputs;

#[component]
pub fn App() -> impl IntoView {
    let input_outline_text = create_rw_signal(InputTextState::new("First name")
        .with_input_type(InputTextType::Outline));

    let input_underline_text = create_rw_signal(InputTextState::new("Last name")
        .with_input_type(InputTextType::Underline));


    view! {
        <form>
            <InputText state=input_outline_text />
            <InputText state=input_underline_text />
        </form>

        <p class="text-gray-500">{input_underline_text().value}</p>
    }
}