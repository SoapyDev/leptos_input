use leptos::{component, view, Children, IntoView};

#[component]
pub fn Row(
    children: Children
) -> impl IntoView {
    
    view!{
        <div class="row">
            {children()}
        </div>
    }
}