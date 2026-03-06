
import { useEffect, useState, useCallback } from 'react';
import { useRouter } from 'next/navigation';

// Define the shape of the WASM auth object
interface KeycloakWasmAuth {
  module: any;
  loading: boolean;
  error: Error | null;
}

export interface UserInfo {
  name: string;
  username: string;
  email: string;
  isLoggedIn: boolean;
}

// Extend the Window interface to include our custom properties
declare global {
  interface Window {
    initKeycloakWasm: () => Promise<void>;
    keycloakWasmAuth: KeycloakWasmAuth;
  }
}

const CONFIG = {
  issuer: 'http://localhost:8888/realms/hackandlearn',
  clientId: 'hackandlearn-client',
  redirectUri: 'http://localhost:8080/callback',
  scope: 'openid profile email',
  challenge: 'S256',
};

export function useKeycloakWasm() {
  const [auth, setAuth] = useState<KeycloakWasmAuth | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);
  const router = useRouter();

  const extractUserInfo = useCallback((token: string): UserInfo => {
    try {
      const parts = token.split('.');
      if (parts.length !== 3) throw new Error('Invalid JWT');
      
      let payload = parts[1].replace(/-/g, '+').replace(/_/g, '/');
      while (payload.length % 4) payload += '=';
      
      const payloadObj = JSON.parse(atob(payload));
      return {
        name: payloadObj.name || payloadObj.preferred_username || 'Unknown User',
        username: payloadObj.preferred_username || payloadObj.sub || 'unknown',
        email: payloadObj.email || 'no-email@example.com',
        isLoggedIn: true
      };
    } catch (err) {
      console.error('[KeyCloak] Failed to extract user info:', err);
      return { name: 'Unknown User', username: 'unknown', email: 'no-email@example.com', isLoggedIn: true };
    }
  }, []);

  const refreshUserInfo = useCallback(() => {
    const localTokens = localStorage.getItem('keycloak_tokens');
    const sessionToken = sessionStorage.getItem('keycloak_access_token');
    
    let token = null;
    if (localTokens) {
      try {
        const parsed = JSON.parse(localTokens);
        token = parsed.id_token || parsed.access_token;
      } catch (e) {}
    }
    
    if (!token && sessionToken) {
      token = sessionToken;
      localStorage.setItem('keycloak_tokens', JSON.stringify({
        access_token: sessionToken,
        id_token: sessionToken,
        token_type: 'Bearer'
      }));
    }

    if (token) {
      setUserInfo(extractUserInfo(token));
    } else {
      setUserInfo(null);
    }
  }, [extractUserInfo]);

  useEffect(() => {
    const initializeWasm = async () => {
      try {
        if (typeof window.initKeycloakWasm === 'function') {
          await window.initKeycloakWasm();
        }
        
        const wasmAuth = await new Promise<KeycloakWasmAuth>((resolve, reject) => {
          const interval = setInterval(() => {
            const currentAuth = window.keycloakWasmAuth;
            if (currentAuth && !currentAuth.loading) {
              clearInterval(interval);
              currentAuth.error ? reject(currentAuth.error) : resolve(currentAuth);
            }
          }, 100);
          setTimeout(() => {
            clearInterval(interval);
            reject(new Error('WASM initialization timed out'));
          }, 10000);
        });
        
        setAuth(wasmAuth);
        refreshUserInfo();
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      }
    };
    
    initializeWasm();
  }, [refreshUserInfo]);

  const login = async () => {
    if (!auth?.module) return;
    try {
      const { JsLoginParams, wasm_login } = auth.module;
      const params = new JsLoginParams(CONFIG.issuer, CONFIG.clientId, CONFIG.redirectUri);
      params.with_scope(CONFIG.scope);
      params.with_challenge(CONFIG.challenge);
      await wasm_login(params);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const handleRedirectCallback = async () => {
    if (!auth?.module) {
      setError('WASM module not ready');
      return;
    }
    
    try {
      const urlParams = new URLSearchParams(window.location.search);
      if (!urlParams.get('code') || !urlParams.get('state')) {
        throw new Error('Missing code or state');
      }
      
      const { JsLoginParams, wasm_handle_redirect_callback } = auth.module;
      const params = new JsLoginParams(CONFIG.issuer, CONFIG.clientId, CONFIG.redirectUri);
      const accessToken = await wasm_handle_redirect_callback(params);
      
      localStorage.setItem('keycloak_tokens', JSON.stringify({
        access_token: accessToken,
        id_token: accessToken,
        refresh_token: 'mock-refresh-token',
        expires_in: 3600,
        token_type: 'Bearer'
      }));
      
      router.push('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setTimeout(() => router.push('/?error=auth_failed'), 3000);
    }
  };

  const logout = () => {
    localStorage.removeItem('keycloak_tokens');
    sessionStorage.removeItem('keycloak_access_token');
    setUserInfo(null);
    const logoutUrl = `${CONFIG.issuer}/protocol/openid-connect/logout`;
    const redirectUri = encodeURIComponent('http://localhost:8080/');
    window.location.href = `${logoutUrl}?redirect_uri=${redirectUri}`;
  };

  return { auth, error, userInfo, login, logout, handleRedirectCallback };
}