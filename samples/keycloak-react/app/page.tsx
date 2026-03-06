import React from 'react';
import LoginButton from '../components/LoginButton';
import { UserProfile } from '../components/UserProfile';

export default function Home() {
  return (
    <main className="min-h-screen p-24">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-8">Keycloak WASM Auth Demo</h1>
        
        <div className="mb-8">
          <p className="text-lg mb-4">Click the button below to login with Keycloak:</p>
          <div className="flex flex-col items-start gap-4">
            <LoginButton />
            <UserProfile />
          </div>
        </div>
      </div>
      
      <div className="mt-12">
        <h2 className="text-2xl font-semibold mb-4">About</h2>
        <p className="text-gray-600">This is a React demo using the Keycloak WASM authentication library.</p>
      </div>
    </main>
  );
}