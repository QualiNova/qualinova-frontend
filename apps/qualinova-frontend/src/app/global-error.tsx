'use client';

import React from 'react';
import Link from 'next/link';

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <html lang="en">
      <body className="bg-[#09090B] text-white flex flex-col items-center justify-center min-h-screen px-4">
        <h1 className="text-4xl font-bold mb-4">Something went wrong!</h1>
        <p className="text-gray-400 mb-8 text-center max-w-md">
          {error.message || 'An unexpected error occurred. Please try again later.'}
        </p>
        <div className="flex gap-4">
          <button
            onClick={reset}
            className="bg-blue-600 hover:bg-blue-700 text-white py-2 px-6 rounded-md transition-colors"
          >
            Try again
          </button>
          <Link
            href="/"
            className="bg-gray-700 hover:bg-gray-600 text-white py-2 px-6 rounded-md transition-colors"
          >
            Return to Home
          </Link>
        </div>
      </body>
    </html>
  );
}