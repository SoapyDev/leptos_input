
use leptos::{component, view, Children, IntoView};
#[component]
pub fn Grid(
    children: Children
) -> impl IntoView {
    view!{
        <div class="grid">
            {children()}
        </div>
    }
}