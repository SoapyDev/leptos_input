use leptos::MaybeSignal;
use leptos::*;
use uuid::Uuid;

#[derive(PartialEq, Clone, Copy)]
pub enum PasswordInputStyle {
    Underline,
    Outline,
    Rounded,
}

/// The level of validation for the password
#[derive(Clone)]
pub enum PasswordValidationLevel {
    /// The password must be at least 8 characters long, and contain at least one lowercase letter, one uppercase letter.
    Weak,
    /// The password must be at least 8 characters long, and contain at least one lowercase letter, one uppercase letter, one number and one special character [ !\"'@#$%^&-_=+/?.,:;`&^|* ].
    Medium,
    /// The password must be at least 12 characters long, and contain at least one lowercase letter, one uppercase letter, one number and one special character [ !\"'@#$%^&-_=+/?.,:;`&^|* ].
    Strong,
    /// Custom validation. Contains a function that returns a boolean, or the error message.
    Custom(fn(&str) -> bool, String),
}

impl PasswordValidationLevel {
    const SPECIAL_CHARACTERS: [&'static str; 23] = [
        "!", "'", "\"", "@", "#", "$", "%", "-", "_", "~", "+", "=", "?", ",", ".", "/", ":", ";",
        "`", "&", "^", "|", "*",
    ];
    fn validate(&self, val: &str) -> bool {
        match self {
            PasswordValidationLevel::Weak => Self::password_is_week(val, 8),
            PasswordValidationLevel::Medium => Self::password_is_medium(val, 8),
            PasswordValidationLevel::Strong => Self::password_is_strong(val, 12),
            PasswordValidationLevel::Custom(func, _) => func(val),
        }
    }

    fn display_message(&self) -> String {
        match self {
            PasswordValidationLevel::Weak => String::from("The password must be at least 8 characters long, and contain at least one lowercase letter, one uppercase letter."),
            PasswordValidationLevel::Medium => String::from("The password must be at least 8 characters long, and contain at least one lowercase letter, one uppercase letter, one number and one special character [ !\"'@#$%^&-_=+/?.,:;`&^|* ]."),
            PasswordValidationLevel::Strong => String::from("The password must be at least 12 characters long, and contain at least one lowercase letter, one uppercase letter, one number and one special character [ !\"'@#$%^&-_=+/?.,:;`&^|* ]."),
            PasswordValidationLevel::Custom(_, message) => message.clone(),
        }
    }

    fn password_is_week(val: &str, len: usize) -> bool {
        if !Self::is_minmal_len_or_more(val, len) {
            return false;
        } else if !Self::contains_lowercase(val) {
            return false;
        } else if !Self::contains_uppercase(val) {
            return false;
        }

        true
    }

    fn password_is_medium(val: &str, len: usize) -> bool {
        if !Self::password_is_week(val, len) {
            return false;
        } else if !Self::contains_number(val) {
            return false;
        } else if !Self::contains_special_character(val) {
            return false;
        }

        true
    }

    fn password_is_strong(val: &str, len: usize) -> bool {
        if !Self::password_is_medium(val, len) {
            return false;
        }

        true
    }
    fn is_minmal_len_or_more(val: &str, len: usize) -> bool {
        if val.len() < len {
            return false;
        }
        true
    }

    fn contains_lowercase(val: &str) -> bool {
        val.chars().any(|c| c.is_lowercase())
    }

    fn contains_uppercase(val: &str) -> bool {
        val.chars().any(|c| c.is_uppercase())
    }

    fn contains_number(val: &str) -> bool {
        val.chars().any(|c| c.is_numeric())
    }

    fn contains_special_character(val: &str) -> bool {
        Self::SPECIAL_CHARACTERS.iter().any(|c| val.contains(c))
    }
}

/// A password input that will validate the password based on a given validation function
#[component]
pub fn InputPassword(
    /// The tracked value
    password: RwSignal<String>,
    /// Whether or not the input is required, defaults to `true`
    #[prop(default = true)]
    required: bool,
    /// Whether or not the input is disabled, defaults to `false`
    #[prop(default = MaybeSignal::from(false))]
    disabled: MaybeSignal<bool>,
    /// The style of the input
    #[prop(default = PasswordInputStyle::Underline)]
    style: PasswordInputStyle,
    /// The validation function
    #[prop(default = PasswordValidationLevel::Strong)]
    validate_change: PasswordValidationLevel,
    /// Label for the input, defaults to `Password`
    #[prop(default = MaybeSignal::from(String::from("Password")))]
    label: MaybeSignal<String>,
) -> impl IntoView {
    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    let id = Uuid::new_v4().to_string();
    let id = move || id.clone();

    let label = label.get();
    let label = move || label.clone();

    let error_message = create_rw_signal(String::new());

    let validator = validate_change.clone();
    let _ = watch(
        move || (password.get()),
        move |password, _, _| {
            if required && password.is_empty() {
                error_message.update(|v| *v = String::from("This field is required"));
            } else if !validator.validate(&password) {
                error_message.update(|v| *v = validator.display_message());
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
                type="password"
                autocomplete="password"
                id=id()
                class="input"
                class:outline = style == PasswordInputStyle::Outline || style == PasswordInputStyle::Rounded
                class:rounded = style == PasswordInputStyle::Rounded
                class:underline = style == PasswordInputStyle::Underline
                class=("valid-input", move || is_valid_change.get())
                class=("invalid-input", move || is_invalid_change.get())
                placeholder=" "
                prop:value=password
                required=move || if required {Some(true)} else {None}
                disabled=move || disabled
                on:input=move |e| {
                    let val = event_target_value(&e);
                    password.update(|v| *v = val);
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
                class:outline = style == PasswordInputStyle::Outline || style == PasswordInputStyle::Rounded
                class:underline = style == PasswordInputStyle::Underline
            >
                {label}
            </label>
            <p class="input-error" class=("show-error", move || is_invalid_change.get() )>
                {error_message}
            </p>
        </div>
    }
}
