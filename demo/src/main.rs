mod app;
mod components;

use leptos::mount_to_body;
use app::App;

fn main() {
    mount_to_body(App)
}
