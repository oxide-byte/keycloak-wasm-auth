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
        <main class="min-h-screen p-24">
            <div class="mb-8">
                <h1 class="text-3xl font-bold mb-8">"Keycloak WASM Auth Demo"</h1>

                <div class="mb-8">
                    <p class="text-lg mb-4">"Click the button below to login with Keycloak:"</p>
                    <div class="flex flex-col items-start gap-4">
                        <LoginButton />
                    </div>
                </div>
            </div>

            <div class="mt-12">
                <h2 class="text-2xl font-semibold mb-4">"About"</h2>
                <p class="text-gray-600">"This is a Leptos demo using the Keycloak WASM authentication library."</p>
            </div>

            <KeycloakAccessAdmin>
                 <div class="mt-8 p-4 bg-green-50 border border-green-200 rounded-lg">
                    <p class="text-xl font-semibold text-green-800">"ADMIN Section"</p>
                    <p class="text-green-600">"You are seeing this because you have the admin role."</p>
                </div>
            </KeycloakAccessAdmin>
        </main>
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