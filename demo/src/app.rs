use leptos_inputs::PasswordInputStyle;
use leptos_inputs::PasswordValidationLevel;
use leptos_inputs::{AddressInputStyle, InputAddress};
use leptos_inputs::InputPassword;
use leptos::*;
use leptos_meta::{provide_meta_context};
use leptos_inputs::{InputEmail, EmailInputStyle, InputText, TextInputStyle};
use leptos_inputs::{GlobalThemeProvider, ThemeToggler};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let value = create_rw_signal(String::new());
    let label_1 = "Input text".to_string();
    let label_2 = "Input text".to_string();
    let label_3 = "Input text".to_string();

    let custom_validation = move |val: &str| !val.is_empty();
    let custom_message = String::from("A password cannot be empty");
    
    let suggestions = Some(
        vec![
            "Password".to_string(),
            "Password2".to_string(),
            "Password3".to_string(),
            "Password4".to_string(),
            "Password5".to_string(),
            "Password6".to_string(),
            "Password7".to_string(),
        ]
    );

    view! {
        <GlobalThemeProvider>
        
            <ThemeToggler />
        
            <InputText label=label_1.into() value=value required=true style=TextInputStyle::Outline/>
            <InputText label=label_2.into() value=value required=true style=TextInputStyle::Underline/>
            <InputText label=label_3.into() value=value required=true style=TextInputStyle::Rounded/>
            
            <InputEmail email=value style=EmailInputStyle::Outline/>
            <InputEmail email=value required=true style=EmailInputStyle::Underline/>
            
            
            <InputPassword password=value validate_change=PasswordValidationLevel::Weak style=PasswordInputStyle::Outline/>
            <InputPassword password=value required=true validate_change=PasswordValidationLevel::Weak style=PasswordInputStyle::Underline/>

            <InputPassword password=value validate_change=PasswordValidationLevel::Medium style=PasswordInputStyle::Outline/>
            <InputPassword password=value required=true validate_change=PasswordValidationLevel::Medium style=PasswordInputStyle::Underline/>
            
            <InputPassword password=value validate_change=PasswordValidationLevel::Strong style=PasswordInputStyle::Outline/>
            <InputPassword password=value required=true validate_change=PasswordValidationLevel::Strong style=PasswordInputStyle::Underline/>
            
            <InputPassword password=value validate_change=PasswordValidationLevel::Custom(custom_validation, custom_message.clone()) style=PasswordInputStyle::Outline/>
            <InputPassword password=value required=true validate_change=PasswordValidationLevel::Custom(custom_validation, custom_message) style=PasswordInputStyle::Underline/>
            
            <InputAddress address=value style=AddressInputStyle::Rounded suggestions=suggestions.into()/>
            
        </GlobalThemeProvider>
    }
}
