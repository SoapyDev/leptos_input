use leptos::component;
use leptos::MaybeSignal;
use leptos::*;
use uuid::Uuid;

#[derive(PartialEq, Clone, Copy)]
pub enum TextInputStyle {
    Underline,
    Outline,
    Rounded,
}

#[component]
pub fn InputText(
    /// The tracked value
    value: RwSignal<String>,
    /// Whether or not the input is required, defaults to `false`
    #[prop(default = false)]
    required: bool,
    /// Whether or not the input is disabled, defaults to `false`
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,
    /// The label of the input
    #[prop(default = MaybeSignal::from(String::new()))]
    label: MaybeSignal<String>,
    /// A function to validate the input value on each stroke
    /// Block the entry if the input is invalid
    #[prop(optional, default=None, into)]
    validate_input: Option<fn(&str) -> bool>,
    /// A function to validate the input value on change.
    /// Will render color the input box.
    /// Green if valid, red if invalid
    #[prop(optional, default=None, into)]
    validate_change: Option<fn(&str) -> bool>,
    /// The error message to display when the change is invalid
    #[prop(optional, into)]
    error_message: RwSignal<String>,
    /// The style to be applied to the input
    #[prop(default = TextInputStyle::Underline)]
    style: TextInputStyle,
) -> impl IntoView {
    let id = Uuid::new_v4().to_string();
    let id = move || id.clone();

    let label = label.get();
    let label = move || label.clone();

    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    let _ = watch(
        move || value.get(),
        move |value, _, _| {
            let validator = validate_change.unwrap_or(|_| true);
            if required && value.is_empty() {
                is_valid_change.update(|v| *v = false);
                is_invalid_change.update(|v| *v = true);
                error_message.update(|v| *v = String::from("This field is required"));
            } else if !validator(&value) {
                is_valid_change.update(|v| *v = false);
                is_invalid_change.update(|v| *v = true);
            } else {
                is_valid_change.update(|v| *v = true);
                is_invalid_change.update(|v| *v = false);
            }
        },
        false,
    );

    view! {
        <div class="input-group">
            <input
                type="text"
                id=id()
                class="input"
                class:outline= style == TextInputStyle::Outline || style == TextInputStyle::Rounded
                class:rounded= style == TextInputStyle::Rounded
                class:underline= style == TextInputStyle::Underline
                class=("valid-input", move || is_valid_change.get())
                class=("invalid-input", move || is_invalid_change.get())
                placeholder=" "
                prop:value=value
                required=move || if required {Some(true)} else {None}
                disabled=move || disabled
                on:input=move |e| {
                    let val = event_target_value(&e);
                    if validate_input.is_some() {
                        if !validate_input.unwrap()(&val) {
                            return;
                        }
                    }
                    value.update(|v| *v = val);
                }
                on:focusout=move |e|{
                    if required && event_target_value(&e).is_empty() {
                        error_message.update(|v| *v = String::from("This field is required"));
                        is_invalid_change.set(true);
                    }
                }
            />
            <label for=id() class="input-label"
                class:outline = {style  == TextInputStyle::Outline || style == TextInputStyle::Rounded}
                class:underline = {style == TextInputStyle::Underline}
            >
                {label}
            </label>
            <p class="input-error" class=("show-error", move || is_invalid_change.get() )>{error_message}</p>
        </div>
    }
}
