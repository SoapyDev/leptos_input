use leptos::*;
use leptos::MaybeSignal;

#[component]
pub fn InputTextUnderline(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = ReadSignal::new(false))] disabled: MaybeSignal<bool>,
    #[prop(default = String::new())] label: impl Into<String>,
    #[prop(into)] id:  MaybeSignal<impl Into<String>>
) -> impl IntoView {

    let id = id.get();
    let id = || id.into();

    view! {
        <div class="input-group">
            <input
                type="text"
                id=id
                class="input underline"
                placeholder=" "
                bind:value=move || value
                required=move || required
                disabled=move || disabled
                on:input=move |e| {
                    let val = event_target_value(&e);
                    value.update(|v| *v = val);
                }
            />
            <label
                for=id
                class="input-label underline"
            >
                {move || label}
            </label>
        </div>
    }
}

#[component]
pub fn InputTextOutline(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = ReadSignal::new(false))] disabled: MaybeSignal<bool>,
    #[prop(default = String::new())] label: impl Into<String>,
    #[prop(into)] id:  MaybeSignal<impl Into<String>>,
    #[prop(optional, default=None, into)] validate_input: Option<fn(&str)  -> bool>,
    #[prop(optional, default=None, into)] validate_change: Option<fn(&str)  -> bool>
) -> impl IntoView {
    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    let id = id.get();
    let id = || id.into();

    view! {
        <div 
            class="relative m-4 border rounded-md border-gray-500 dark:border-gray-400 has-[:focus]:border-blue-600 has-[:focus]:dark:border-blue-600"
            class=("invalid-input", is_invalid_change.get())
            class=("valid-input", is_valid_change.get())
        >
            <input
                type="text"
                id=id
                class="block px-2.5 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-1 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
                class=("valid-input", is_valid_change.get())
                class=("invalid-input", is_invalid_change.get())
                placeholder=" "
                bind:value=move || value.get()
                required=move || required
                disabled=move || disabled.get()
                on:input=move |e| {
                    let val = event_target_value(&e);
                    if validate_input.is_some() {
                        if !validate_input.unwrap()(&val) {
                            return;
                        }
                    }
                    value.update(|v| *v = val);
                }
                on:change=move |e| {
                    if validate_change.is_some() {
                        if validate_change.unwrap()(&event_target_value(&e)) {
                            is_valid_change.update(|v| *v = false);
                        }else{
                            is_invalid_change.update(|v| *v = false);
                        }
                    }
                }
            />
            <label
                for=id
                class="absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] px-2 peer-focus:px-2 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-1/2 peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto start-1"
            >
                {move || label}
            </label>
        </div>
    }
}