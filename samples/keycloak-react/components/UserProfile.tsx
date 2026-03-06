'use client';

import { useKeycloakWasm } from '@/app/hooks/useKeycloakWasm';

export function UserProfile() {
  const { userInfo, logout } = useKeycloakWasm();

  if (!userInfo) {
    return null;
  }

  return (
    <div className="flex flex-col sm:flex-row items-center gap-3 bg-white border border-gray-200 rounded-lg p-4 shadow-sm min-w-[250px]">
      <div className="flex items-center gap-3 flex-1 min-w-0">
        <div className="min-w-0">
          <div className="font-medium text-gray-900 truncate">User: {userInfo.name}</div>
          <div className="text-sm text-gray-500 truncate">Mail: {userInfo.email}</div>
        </div>
      </div>
      <button
        onClick={logout}
        className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors text-sm font-medium whitespace-nowrap flex-shrink-0"
      >
        Logout
      </button>
    </div>
  );
}