use leptos::{event_target_value, watch, ReadSignal};
use leptos::{SignalSet, SignalUpdate};
use leptos::{create_signal, For};
use leptos::SignalGet;
use uuid::Uuid;
use leptos::create_rw_signal;
use leptos::{component, view, IntoView, MaybeSignal, RwSignal};
use leptos_use::use_css_var;
#[derive(PartialEq)]
pub enum AddressInputStyle {
    Underline,
    Outline,
    Search,
    Rounded
}
#[component]
pub fn InputAddress(
    /// The tracked value
    #[prop(into)] address: RwSignal<String>,
    /// The label of the input, defaults to `Address`
    #[prop(into, default = MaybeSignal::from(String::from("Address")))] label: MaybeSignal<String>,
    /// Whether or not the input is required, defaults to `false`
    #[prop(default = false)] required: bool,
    /// Whether or not the input is disabled, defaults to `false`
    #[prop(default = MaybeSignal::from(false))] disabled: MaybeSignal<bool>,
    /// The style of the input
    #[prop(default = AddressInputStyle::Underline)] style: AddressInputStyle,
    /// The suggestions of the input
    #[prop(default = MaybeSignal::from(None))] suggestions: MaybeSignal<Option<Vec<String>>>
) -> impl IntoView {

    let error_message = create_rw_signal(String::new());
    
    let _ = watch(move || address.get(), 
          move |address, _, _| {
              if required && address.is_empty() {
                  error_message.update(|v| *v = String::from("This field is required"));
              }
          }, false
    );
    
    let is_invalid_change = create_rw_signal(false);
    let is_valid_change = create_rw_signal(false);

    let id = Uuid::new_v4().to_string();
    let id = move || id.clone();

    let label = label.get();
    let label = move || label.clone();
    
    
    let selected = create_rw_signal(0usize);
    
    let (suggestions, set_suggestions) = create_signal(suggestions.get());
    
    view!{
        <div class="input-group">
            <input
                type="text"
                id=id()
                class="input"
                class:outline = style == AddressInputStyle::Outline || style == AddressInputStyle::Rounded
                class:rounded = style == AddressInputStyle::Rounded
                class:search = style == AddressInputStyle::Search
                class:underline = style == AddressInputStyle::Underline
                class=("valid-input", move || is_valid_change.get())
                placeholder=" "
                prop:value=address
                required=move || if required {Some(true)} else {None}
                disabled=move || disabled
                on:input=move |e| {
                    let val = event_target_value(&e);
                    if required && val.is_empty() {
                        is_invalid_change.set(false);
                    }
                    address.update(|v| *v = val);
                }
                on:keydown=move |e|{
                    if suggestions.get().is_some() {
                        let s = suggestions.get().unwrap();
                        let len = s.len();
                        if e.key() == "ArrowDown" {
                            if selected.get() < len - 1 {
                                selected.update(|v| *v += 1);
                            }
                        }
                        if e.key() == "ArrowUp" {
                            if selected.get() > 0 {
                                selected.update(|v| *v -= 1);
                            }
                        }
                        if e.key() == "Enter" {
                            e.prevent_default();
                            let suggestion = s.get(selected.get()).unwrap();
                            address.update(|v| *v = suggestion.clone());
                            set_suggestions.set(None);
                        }
                    }
                }
                />
            <label 
                for=id() 
                class="input-label"
                class:outline = style == AddressInputStyle::Outline || style == AddressInputStyle::Rounded
                class:search = style == AddressInputStyle::Search
                class:underline = style == AddressInputStyle::Underline
            >
                {label}
            </label>
            <Suggestions value=address suggestions=suggestions selected=selected style=style />
            <p class="input-error" class=("show-error", move || is_invalid_change.get() )>
                {error_message}
            </p>
        </div>
    }
}

#[component]
fn Suggestions(
    value: RwSignal<String>,
    suggestions: ReadSignal<Option<Vec<String>>>,
    selected: RwSignal<usize>,
    style : AddressInputStyle
) -> impl IntoView {
        
        view!{

            {move || if suggestions.get().is_some() {
                let len = suggestions.get().unwrap().len().min(5);
                let (_list_len, set_list_len) = use_css_var("--list-size");
                set_list_len.set(len.to_string());
                
                
                view!{
                    <div class="suggestions"
                        class:rounded=style == AddressInputStyle::Rounded
                    >
                    <ul>
                        <For
                            each=move || suggestions.get().unwrap().into_iter().enumerate()
                            key=|s| s.clone()
                            children=move |(i, val)| {
                        
                            let suggestion = val.clone();

                                view!{

                                    <li class:selected=move || selected.get() == i>
                                        <button
                                            type="button"
                                            class="suggestion"
                                            on:click=move |_| value.set(suggestion.clone())
                                        >
                                            {val}
                                        </button>
                                    </li>
                                }
                            }
                        />
                    </ul>
                    </div>
                }.into_view()
            }else{
                view!{}.into_view()
            }}
        }
}