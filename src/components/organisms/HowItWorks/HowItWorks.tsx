import React from 'react';

type StepProps = {
  number: number;
  title: string;
  description: string;
};


const ProcessStep: React.FC<StepProps> = ({ number, title, description }) => {
  return (
    <div className="flex flex-col items-center px-4">
      <div className="bg-blue-500 rounded-full w-10 h-10 flex items-center justify-center text-white font-bold mb-4">
        {number}
      </div>
      <h3 className="text-white text-lg font-bold mb-2">{title}</h3>
      <p className="text-[#9CA3AF] text-center text-sm">{description}</p>
    </div>
  );
};

// Button Component
type ButtonProps = {
  primary?: boolean;
  children: React.ReactNode;
  onClick?: () => void;
};

const Button: React.FC<ButtonProps> = ({ primary = false, children, onClick }) => {
  return (
    <button
      onClick={onClick}
      className={`px-6 py-3 rounded font-medium ${
        primary
          ? 'bg-gray-800 text-white hover:bg-gray-700'
          : 'bg-transparent border border-white text-white hover:bg-white hover:text-blue-900'
      } transition-colors duration-300`}
    >
      {children}
    </button>
  );
};
const HowItWorks: React.FC = () => {
  return (
    <div className="min-h-screen flex flex-col">
      {/* How It Works Section */}
      <section className="bg-gray-900 py-16">
        <div className="container mx-auto px-4">
          <h2 className="text-white text-3xl font-bold text-center mb-2">How It Works</h2>
          <p className=" text-center text-[#9CA3AF] mb-12">
          Our blockchain certification process is simple, secure, and efficient
          </p>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <ProcessStep
              number={1}
              title="Create Certificate"
              description="Fill out the certificate details and generate a new certification"
            />
            <ProcessStep
              number={2}
              title="Blockchain Registration"
              description="Certificate is registered on the Stellar blockchain with a unique identifier"
            />
            <ProcessStep
              number={3}
              title="Verify Anytime"
              description="Certificates can be verified instantly by anyone using our verification tool"
            />
          </div>
        </div>
      </section>

      {/* Get Started Section */}
      <section className="bg-blue-900 py-16 flex-grow">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-white text-3xl font-bold mb-2">Ready to Get Started?</h2>
          <p className="text-[#DBEAFE] mb-8">
          Join organizations worldwide using QualiNova for secure certification
          management
          </p>

          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button primary>Create Your First Certificate</Button>
            <Button>Verify a Certificate</Button>
          </div>
        </div>
      </section>
    </div>
  );
};

export default HowItWorks;