use leptos::*;

#[derive(Debug, Clone)]
pub struct InputTextState {
    pub label: String,
    pub value: RwSignal<String>,
    pub required: bool,
    pub disabled: RwSignal<bool>,
    pub validate_input: fn(&str) -> bool,
    pub validate_change: fn(&str) -> bool,
    pub input_type: InputTextType,
}

impl Default for InputTextState {
    fn default() -> Self {
        Self {
            label: String::new(),
            value: create_rw_signal(String::new()),
            required: false,
            disabled: create_rw_signal(false),
            validate_input: |_: &str| true,
            validate_change: |_: &str| true,
            input_type: InputTextType::default(),
        }
    }
}

impl InputTextState {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn with_validator(self, validator: fn(&str) -> bool) -> Self {
        Self { validate_input: validator, ..self }
    }

    pub fn with_input_type(self, input_type: InputTextType) -> Self {
        Self { input_type, ..self }
    }

    pub fn with_disabled(self, disabled: bool) -> Self {
        Self { disabled: create_rw_signal(disabled), ..self }
    }

    pub fn with_required(self, required: bool) -> Self {
        Self { required, ..self }
    }

    pub fn with_value(self, value: &str) -> Self {
        Self {
            value: create_rw_signal(value.to_string()),
            ..self
        }
    }

    pub fn with_label(self, label: &str) -> Self {
        Self {
            label: label.to_string(),
            ..self
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum InputTextType {
    #[default]
    Underline,
    Outline,
}


#[component]
pub fn InputText(state: RwSignal<InputTextState>) -> impl IntoView {
    match state().input_type {
        InputTextType::Underline => view! { <InputTextUnderline state /> },
        InputTextType::Outline => view! { <InputTextOutline state /> },
    }
}

#[component]
fn InputTextUnderline(state: RwSignal<InputTextState>) -> impl IntoView {
    view! {
        <div class="relative z-0 m-4">
            <input
                type="text"
                id="floating_standard"
                class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                placeholder=" "
                bind:value=move || state().value.get()
                required=move || state().required
                disabled=move || state().disabled.get()
                on:input=move |e| {
                    let value = event_target_value(&e);
                    state().value.update(|v| *v = value);
                }
            />
            <label
                for="floating_standard"
                class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
            >
                {move || state().label}
            </label>
        </div>
    }
}

#[component]
fn InputTextOutline(state: RwSignal<InputTextState>) -> impl IntoView {
    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    view! {
        <div 
            class="relative m-4 border rounded-md border-gray-500 dark:border-gray-400 has-[:focus]:border-blue-600 has-[:focus]:dark:border-blue-600"
            class=("invalid-input", is_invalid_change.get())
            class=("valid-input", is_valid_change.get())
        >
            <input
                type="text"
                id="floating_outlined"
                class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-1 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                class=("valid-input", is_valid_change.get())
                class=("invalid-input", is_invalid_change.get())
                placeholder=" "
                bind:value=move || state().value.get()
                required=move || state().required
                disabled=move || state().disabled.get()
                on:input=move |e| {
                    let value = event_target_value(&e);

                    if !(state().validate_input)(&value) {
                        return;
                    }
                    state().value.update(|v| *v = value);
                }
                on:change=move |e| {
                    if (state().validate_change)(&event_target_value(&e)) {
                        is_valid_change.update(|v| *v = false);
                    }else{
                        is_invalid_change.update(|v| *v = false);
                    }
                }
            />
            <label
                for="floating_outlined"
                class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
            >
                {move || state().label}
            </label>
        </div>
    }
}