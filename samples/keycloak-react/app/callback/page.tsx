'use client';

import { useEffect } from 'react';
import { useKeycloakWasm } from '../hooks/useKeycloakWasm';

export default function CallbackPage() {
  const { auth, error, handleRedirectCallback } = useKeycloakWasm();

  useEffect(() => {
    if (auth) {
      handleRedirectCallback();
    }
  }, [auth, handleRedirectCallback]);

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="text-center">
        {error ? (
          <div className="bg-red-50 border-l-4 border-red-400 p-4 mb-6">
            <div className="flex">
              <div className="flex-shrink-0">
                <svg className="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                </svg>
              </div>
              <div className="ml-3">
                <p className="text-sm text-red-700">{error}</p>
                <p className="text-sm text-red-600 mt-1">Redirecting to home page...</p>
              </div>
            </div>
          </div>
        ) : (
          <>
            <h2 className="text-2xl font-semibold text-gray-800 mb-4">Processing Authentication</h2>
            <p className="text-gray-600 mb-6">Please wait while we complete your login...</p>
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mx-auto"></div>
          </>
        )}
      </div>
    </div>
  );
}