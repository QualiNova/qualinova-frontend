import React from 'react';
import Link from 'next/link';

export default function NotFound() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] px-4">
      <h1 className="text-4xl font-bold text-white mb-4">404 - Page Not Found</h1>
      <p className="text-gray-400 mb-8 text-center max-w-md">
        The page you are looking for might have been removed, had its name changed, or is temporarily unavailable.
      </p>
      <Link
        href="/"
        className="bg-blue-600 hover:bg-blue-700 text-white py-2 px-6 rounded-md transition-colors"
      >
        Return to Home
      </Link>
    </div>
  );
}