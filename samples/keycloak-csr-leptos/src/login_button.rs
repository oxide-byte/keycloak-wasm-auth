use crate::auth_config;
use keycloak_wasm_auth::{Challenge, LoginParams};
use leptos::__reexports::wasm_bindgen_futures;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn LoginButton() -> impl IntoView {

    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";
    let on_login = move |_| {

        // Spawn async task for login
        wasm_bindgen_futures::spawn_local(async move {
            log!("[KeyCloak] Starting login flow...");

            // Configure KeyCloak login parameters
            let params = LoginParams::new(
                auth_config::KEYCLOAK_ISSUER.to_string(),
                auth_config::KEYCLOAK_CLIENT_ID.to_string(),
                auth_config::KEYCLOAK_REDIRECT_URI.to_string(),
            )
                .with_scope(auth_config::KEYCLOAK_SCOPE.to_string())
                .with_challenge(Challenge::S256);

            log!("[KeyCloak] Login params: issuer={}, client_id={}, redirect_uri={}",
                params.issuer, params.client_id, params.redirect_uri);

            // This will redirect to KeyCloak, so the code after this won't execute
            match keycloak_wasm_auth::login(params).await {
                Ok(_) => {
                    log!("[KeyCloak] Redirecting to KeyCloak...");
                    // This line won't be reached because login() redirects the browser
                }
                Err(e) => {
                    log!("[KeyCloak] ❌ Login initiation failed: {}", e);
                }
            }
        });
    };

    view! {
        <button on:click=on_login class=button_class>
            "LOGIN"
        </button>
    }
}