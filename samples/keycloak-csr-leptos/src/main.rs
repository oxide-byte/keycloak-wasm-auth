mod keycloak_access_admin;
mod keycloak_catcher;
mod auth_config;
mod login_button;

use crate::keycloak_access_admin::KeycloakAccessAdmin;
use crate::keycloak_catcher::{GlobalState, KeyCloakCatcher};
use crate::login_button::LoginButton;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {

    view! {
        <div>
            <p> Hello World </p>
        </div>

        <br/>

        <div>
            <LoginButton/>
        </div>

        <br/>

        <KeycloakAccessAdmin>
             <div>
                <p class="text-3xl">ADMIN Section</p>
            </div>
        </KeycloakAccessAdmin>
    }
}

fn main() {
    wasm_tracing::set_as_global_default();
    // Initialize GlobalState
    let state = Store::new(GlobalState::default());

    mount_to_body(move || {
        provide_context(state.clone());

        view! {
            <KeyCloakCatcher/>
            <App/>
        }
    })
}