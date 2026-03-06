'use client';

import React from 'react';
import { useKeycloakWasm } from '@/app/hooks/useKeycloakWasm';

export default function LoginButton() {
  const { auth, error, login } = useKeycloakWasm();

  if (error) {
    return (
      <div className="text-red-500 p-4 bg-red-50 rounded-lg">
        <p className="font-medium">Error loading WASM module:</p>
        <p className="text-sm mt-1">{error}</p>
      </div>
    );
  }

  if (!auth) {
    return (
      <button
        disabled
        className="bg-blue-400 px-8 py-3 text-white rounded-lg font-medium cursor-not-allowed"
      >
        Loading WASM...
      </button>
    );
  }

  return (
    <button
      onClick={login}
      className="bg-blue-700 hover:bg-blue-800 px-8 py-3 text-white rounded-lg font-medium transition-colors"
    >
      LOGIN WITH KEYCLOAK
    </button>
  );
}