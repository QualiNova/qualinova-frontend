import React from 'react';
import Link from 'next/link';

export default function ServerError() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] px-4">
      <h1 className="text-4xl font-bold text-white mb-4">500 - Server Error</h1>
      <p className="text-gray-400 mb-8 text-center max-w-md">
        Something went wrong on our end. We're working to fix the issue.
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