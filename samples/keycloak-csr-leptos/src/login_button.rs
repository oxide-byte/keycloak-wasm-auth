use crate::auth_config;
use crate::keycloak_catcher::{GlobalState, GlobalStateStoreFields};
use keycloak_wasm_auth::{Challenge, LoginParams, LogoutParams};
use leptos::__reexports::wasm_bindgen_futures;
use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::{Patch, Store};

#[component]
pub fn LoginButton() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let is_authenticated = state.is_authenticated();

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
        <Show
            when=move || !is_authenticated.get()
            fallback=move || view! { <UserProfile /> }
        >
            <button
                on:click=on_login
                class="bg-blue-700 hover:bg-blue-800 px-8 py-3 text-white rounded-lg font-medium transition-colors"
            >
                "LOGIN WITH KEYCLOAK"
            </button>
        </Show>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let name = state.name();
    let email = state.email();

    let on_logout = move |_| {
        // Clear local state first
        state.token().patch(None);
        state.user_id().patch(None);
        state.email().patch(None);
        state.username().patch(None);
        state.name().patch(None);
        state.roles().patch(Vec::new());
        state.is_authenticated().patch(false);

        // Spawn async task for OIDC logout
        wasm_bindgen_futures::spawn_local(async move {
            log!("[KeyCloak] Initiating Keycloak logout...");

            let mut params = LogoutParams::new(auth_config::KEYCLOAK_ISSUER.to_string())
                .with_post_logout_redirect_uri(auth_config::KEYCLOAK_REDIRECT_URI.to_string());

            // Add ID token hint if available
            if let Ok(id_token) = keycloak_wasm_auth::retrieve_id_token() {
                params = params.with_id_token_hint(id_token);
            }

            match keycloak_wasm_auth::logout(params).await {
                Ok(_) => {
                    log!("[KeyCloak] Logged out successfully");
                }
                Err(e) => {
                    log!("[KeyCloak] ❌ Logout failed: {}", e);
                    // Fallback to home redirect if OIDC logout fails
                    if let Some(window) = leptos::web_sys::window() {
                        let _ = window.location().set_href("/");
                    }
                }
            }
        });
    };

    view! {
        <div class="flex flex-col sm:flex-row items-center gap-3 bg-white border border-gray-200 rounded-lg p-4 shadow-sm min-w-[250px]">
          <div class="flex items-center gap-3 flex-1 min-w-0">
            <div class="min-w-0">
              <div class="font-medium text-gray-900 truncate">
                "User: " {move || name.get().unwrap_or_else(|| "Unknown".to_string())}
              </div>
              <div class="text-sm text-gray-500 truncate">
                "Mail: " {move || email.get().unwrap_or_else(|| "N/A".to_string())}
              </div>
            </div>
          </div>
          <button
            on:click=on_logout
            class="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-sm font-medium whitespace-nowrap flex-shrink-0"
          >
            "Logout"
          </button>
        </div>
    }
}