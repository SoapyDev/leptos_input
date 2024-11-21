use leptos::{component, view};
use leptos::{Children, IntoView, MaybeSignal};

#[derive(PartialEq, Clone, Copy)]
pub enum FormBoxStyle {
    SimpleLogin,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Padding {
    None,
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Level {
    Dp1,
    Dp2,
    Dp3,
    Dp4,
    Dp5,
    Dp6,
    Dp7,
}
#[component]
pub fn FormBox(
    #[prop(default = FormBoxStyle::SimpleLogin)] style: FormBoxStyle,
    #[prop(default = Padding::Medium)] padding: Padding,
    #[prop(default = MaybeSignal::from(Some(String::from("Login")))
    )]
    title: MaybeSignal<Option<String>>,
    #[prop(into, optional)] logo_src: Option<MaybeSignal<String>>,
    #[prop(into, optional)] logo_alt: Option<MaybeSignal<String>>,
    #[prop(into, optional)] footer_content: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="form"
            class:simple-login=style == FormBoxStyle::SimpleLogin
            class:padding-small=padding == Padding::Small
            class:padding-medium=padding == Padding::Medium
            class:padding-large=padding == Padding::Large
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
