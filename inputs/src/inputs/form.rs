use leptos::{Children, IntoView, MaybeSignal};
use leptos::{component, view};

#[derive(PartialEq, Clone, Copy)]
pub enum FormBoxStyle {
    SimpleLogin
}
#[component]
pub fn FormBox(
    #[prop(default = FormBoxStyle::SimpleLogin)] style: FormBoxStyle,
    #[prop(into, default = MaybeSignal::from(String::from("Login")))] title: MaybeSignal<String>,
    #[prop(into, optional)] logo_src: Option<MaybeSignal<String>>,
    #[prop(into, optional)] logo_alt: Option<MaybeSignal<String>>,
    #[prop(into, optional)] footer_content: Option<MaybeSignal<String>>,
    children: Children
) -> impl IntoView {
    
    view!{
        <div 
            class="form"
            class:simple-login=style == FormBoxStyle::SimpleLogin
        >
            <header class="form-header">
                <img src={logo_src} alt={logo_alt}/>
                <h2>{title}</h2>
            </header>
        
            {children()}
        
            <footer class="form-footer">
                {footer_content}
            </footer>
        </div>
    }
}