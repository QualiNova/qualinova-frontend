import React from 'react';
import { NextPage } from 'next';
import Link from 'next/link';

interface ErrorProps {
  statusCode?: number;
}

const Error: NextPage<ErrorProps> = ({ statusCode }: ErrorProps) => {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] px-4">
      <h1 className="text-4xl font-bold text-white mb-4">
        {statusCode ? `Error ${statusCode}` : 'An error occurred'}
      </h1>
      <p className="text-gray-400 mb-8 text-center max-w-md">
        {statusCode === 404
          ? 'The page you are looking for might have been removed, had its name changed, or is temporarily unavailable.'
          : 'Something went wrong on our end. We\'re working to fix the issue.'}
      </p>
      <Link
        href="/"
        className="bg-blue-600 hover:bg-blue-700 text-white py-2 px-6 rounded-md transition-colors"
      >
        Return to Home
      </Link>
    </div>
  );
};

Error.getInitialProps = ({ res, err }: { res?: any; err?: any }) => {
  const statusCode = res ? res.statusCode : err ? err.statusCode : 404;
  return { statusCode };
};

export default Error;