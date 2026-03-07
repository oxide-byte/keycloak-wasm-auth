use wasm_bindgen::prelude::*;
use keycloak_wasm_auth::{LoginParams, LogoutParams, Challenge, login, logout, handle_redirect_callback};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub struct JsLoginParams {
    inner: LoginParams,
}

#[wasm_bindgen]
impl JsLoginParams {
    #[wasm_bindgen(constructor)]
    pub fn new(issuer: String, client_id: String, redirect_uri: String) -> JsLoginParams {
        let params = LoginParams::new(issuer, client_id, redirect_uri);
        JsLoginParams { inner: params }
    }

    #[wasm_bindgen]
    pub fn with_scope(&mut self, scope: String) {
        self.inner = self.inner.clone().with_scope(scope);
    }

    #[wasm_bindgen]
    pub fn with_challenge(&mut self, challenge: String) {
        let challenge = match challenge.as_str() {
            "S256" => Challenge::S256,
            "plain" => Challenge::Plain,
            _ => Challenge::S256, // default
        };
        self.inner = self.inner.clone().with_challenge(challenge);
    }
}

#[wasm_bindgen]
pub async fn wasm_login(params: JsLoginParams) -> Result<JsValue, JsValue> {
    // Call the login function which will redirect to Keycloak
    // This function should not return as it redirects the browser
    let result = login(params.inner).await;
    
    // If we get here, the redirect failed
    match result {
        Ok(_) => {
            // This should never happen as login() redirects
            Ok(JsValue::from_str("Login initiated successfully"))
        },
        Err(e) => {
            // Return error if redirect failed
            Err(JsValue::from_str(&format!("Login failed: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub async fn wasm_handle_redirect_callback(params: JsLoginParams) -> Result<JsValue, JsValue> {
    // Call the handle_redirect_callback function to exchange code for tokens
    let result = handle_redirect_callback(params.inner).await;
    
    match result {
        Ok(token) => {
            // Return the access token as a string
            Ok(JsValue::from_str(&token))
        },
        Err(e) => {
            // Return error if token exchange failed
            Err(JsValue::from_str(&format!("Callback handling failed: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub struct JsLogoutParams {
    inner: LogoutParams,
}

#[wasm_bindgen]
impl JsLogoutParams {
    #[wasm_bindgen(constructor)]
    pub fn new(issuer: String) -> JsLogoutParams {
        let params = LogoutParams::new(issuer);
        JsLogoutParams { inner: params }
    }

    #[wasm_bindgen]
    pub fn with_post_logout_redirect_uri(&mut self, uri: String) {
        self.inner = self.inner.clone().with_post_logout_redirect_uri(uri);
    }

    #[wasm_bindgen]
    pub fn with_id_token_hint(&mut self, token: String) {
        self.inner = self.inner.clone().with_id_token_hint(token);
    }
}

#[wasm_bindgen]
pub async fn wasm_logout(params: JsLogoutParams) -> Result<JsValue, JsValue> {
    // Call the logout function which will redirect to Keycloak logout page
    let result = logout(params.inner).await;
    
    match result {
        Ok(_) => {
            Ok(JsValue::from_str("Logout initiated successfully"))
        },
        Err(e) => {
            Err(JsValue::from_str(&format!("Logout failed: {}", e)))
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize logging
    console_error_panic_hook::set_once();
    Ok(())
}