

const HeroSection = () => {
  return (
    <div className="min-h-screen bg-gray-900 text-white font-sans">


      <main className="container mx-auto px-16  ">
        <div className="flex flex-col md:flex-row items-center justify-between gap-8 lg:gap-16">
          {/* Left Section - Content */}
          <div className="w-full md:w-1/2 space-y-6">
            {/* Progress Bar */}
            <div className="mb-4">
              <div className="bg-[#1E3A8A] py-2 px-6 rounded-full w-full">
                Blockchain-Powered Certification
              </div>
            </div>

            {/* Main Heading */}
            <h1 className="text-3xl md:text-4xl lg:text-5xl font-bold leading-tight">
              Secure, Verifiable Certifications on the Blockchain
            </h1>

            {/* Description */}
            <p className="text-gray-300 font-light text-2xl leading-10 md:pr-8">
              QualiNova provides tamper-proof certification management powered
              by Stellar blockchain technology. Create, verify, and manage
              certifications with confidence
            </p>

            {/* Call to Action Buttons */}
            <div className="pt-4 flex flex-wrap gap-4">
              <button className="bg-blue-600 hover:bg-blue-700 text-white py-2 px-6 rounded-md transition-colors">
                Try for Free
              </button>
              <button className="border border-gray-600 hover:border-gray-400 text-white py-2 px-6 rounded-md transition-colors">
                See How it Works
              </button>
            </div>
          </div>

          {/* Right Section - Certificate Image */}
          <div className="w-full md:w-1/2 flex justify-center items-center mt-8 md:mt-0 h-screen ">
            <div className="bg-gray-800 rounded-lg w-full max-w-md h-full md:h-[600px] flex items-center justify-center">
              <div className="text-gray-500">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  className="w-8 h-8"
                >
                  <path d="M18 3v4"></path>
                  <path d="M6 3v4"></path>
                  <path d="M3 7h18"></path>
                  <rect x="3" y="7" width="18" height="14" rx="2"></rect>
                  <path d="M8 12h8"></path>
                  <path d="M8 16h8"></path>
                </svg>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};

export default HeroSection;
