use leptos::{component, view, Children, IntoView};

#[component]
pub fn Column(
    children: Children
) -> impl IntoView {

    view!{
        <div class="column">
            {children()}
        </div>
    }
}