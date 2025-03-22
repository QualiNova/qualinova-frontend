// File: components/HowItWorks.tsx
import React from 'react';

const HowItWorks: React.FC = () => {
  const steps = [
    {
      number: 1,
      title: 'Create Certificate',
      description: 'Create, design, and download educational or professional certificates easily',
      iconColor: 'bg-blue-500',
    },
    {
      number: 2,
      title: 'Blockchain Registration',
      description: 'Certificates are registered on the blockchain with a unique and tamper-proof digital signature',
      iconColor: 'bg-blue-600',
    },
    {
      number: 3,
      title: 'Verify Anytime',
      description: 'Certificates can be verified anytime by anyone using our verification tool',
      iconColor: 'bg-blue-700',
    },
  ];

  return (
    <section className="py-16 px-4 bg-gray-900 h-full md:h-screen">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-14">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">How It Works</h2>
          <p className="text-gray-400 max-w-2xl mx-auto">
            Our blockchain certification process is simple, secure, and efficient
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {steps.map((step) => (
            <div key={step.number} className="flex flex-col items-center text-center">
              <div className={`w-12 h-12 rounded-full ${step.iconColor} flex items-center justify-center text-white font-bold text-xl mb-6`}>
                {step.number}
              </div>
              <h3 className="text-xl font-semibold text-white mb-3">{step.title}</h3>
              <p className="text-gray-400 text-sm">{step.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default HowItWorks;