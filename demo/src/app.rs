use leptos::*;
use leptos_inputs::InputPassword;
use leptos_inputs::PasswordValidationLevel;
use leptos_inputs::{AddressInputStyle, InputAddress};
use leptos_inputs::{Button, ButtonAnimation, ButtonSize, ButtonStyle, ButtonColor, PasswordInputStyle};
use leptos_inputs::{EmailInputStyle, InputEmail, InputText, TextInputStyle};
use leptos_inputs::{FormBox, GlobalThemeProvider, ThemeToggler};
use leptos_meta::provide_meta_context;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let value = create_rw_signal(String::new());
    let label_1 = "Input text".to_string();
    let label_2 = "Input text".to_string();
    let label_3 = "Input text".to_string();

    let custom_validation = move |val: &str| !val.is_empty();
    let custom_message = String::from("A password cannot be empty");
    
    let title = Some("Input Demo".to_string());
    
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
        
            <nav>
                <div>
                    <h1>"Input Demo"</h1>
                    <ThemeToggler />
                </div>
            </nav>
        
            <FormBox>
                <form style="display: flex; flex-direction: column;gap: 1.5rem;">
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
                </form>
            </FormBox>
        

            <div>
                <Button text="Submit".to_string().into() /> 
                <Button style=ButtonStyle::RoundedSolid />
                <Button style=ButtonStyle::RoundedOutline />
                <Button style=ButtonStyle::Solid />
                <Button style=ButtonStyle::Outline />
                <Button color=ButtonColor::Primary />
                <Button color=ButtonColor::Secondary />
                <Button color=ButtonColor::Error />
                <Button color=ButtonColor::Success />
                <Button size=ButtonSize::Small />
                <Button size=ButtonSize::Medium />
                <Button size=ButtonSize::Large />
                <Button size=ButtonSize::FullSize />
                <Button animation=ButtonAnimation::Push />
                <Button animation=ButtonAnimation::Fill />
                <Button animation=ButtonAnimation::Float />
            </div>
            
        </GlobalThemeProvider>
    }
}
