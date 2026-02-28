use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {

    view! {
        <div>
            <p> Hello World </p>
        </div>
    }
}

fn main() {
    wasm_tracing::set_as_global_default();
    mount_to_body(App)
}