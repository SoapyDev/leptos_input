use leptos::MaybeSignal;
use leptos::*;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct EmailInput {
    #[validate(email)]
    pub(crate) email: String,
}

#[derive(PartialEq, Clone, Copy)]
pub enum EmailInputStyle {
    Underline,
    Outline,
    Rounded,
}

/// An email input that will validate email based on HTML5 spec RFC 5322
#[component]
pub fn InputEmail(
    /// The tracked value
    email: RwSignal<String>,
    /// Whether or not the input is required, defaults to `false`
    #[prop(default = false)]
    required: bool,
    /// Whether or not the input is disabled, defaults to `false`
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,
    /// The style of the input
    #[prop(default = EmailInputStyle::Underline)]
    style: EmailInputStyle,
) -> impl IntoView {
    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    let id = Uuid::new_v4().to_string();
    let id = move || id.clone();

    let label = move || "Email".to_string();

    let error_message = create_rw_signal(String::new());

    let validate_change = move |email: &str| {
        EmailInput {
            email: email.to_string(),
        }
        .validate()
        .is_ok()
    };

    let _ = watch(
        move || email.get(),
        move |email, _, _| {
            if required && email.is_empty() {
                error_message.update(|v| *v = String::from("This field is required"));
            } else if !validate_change(&email) {
                error_message.update(|v| *v = String::from("Please enter a valid email address."));
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
                autocomplete="email"
                id=id()
                class="input"
                class:outline = style == EmailInputStyle::Outline || style == EmailInputStyle::Rounded
                class:rounded = style == EmailInputStyle::Rounded
                class:underline = style == EmailInputStyle::Underline
                class=("valid-input", move || is_valid_change.get())
                class=("invalid-input", move || is_invalid_change.get())
                placeholder=" "
                prop:value=email
                required=move || if required {Some(true)} else {None}
                disabled=move || disabled
                on:input=move |e| {
                    let val = event_target_value(&e);
                    email.update(|v| *v = val);
                }
                on:focusout=move |e|{
                    if required && event_target_value(&e).is_empty() {
                        error_message.update(|v| *v = String::from("This field is required"));
                        is_invalid_change.set(true);
                        is_valid_change.set(false);
                    }else if !required && event_target_value(&e).is_empty() {
                        is_invalid_change.set(false);
                        is_valid_change.set(true);
                    }
                }
            />
            <label
                for=id()
                class="input-label"
                class:outline = style == EmailInputStyle::Outline || style == EmailInputStyle::Rounded
                class:underline = style == EmailInputStyle::Underline
            >
                {label}
            </label>
            <p class="input-error" class=("show-error", move || is_invalid_change.get() )>
                {error_message}
            </p>
        </div>
    }
}
