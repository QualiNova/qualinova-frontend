import React from "react";
import { Shield } from "lucide-react";
import { Inter } from "next/font/google";

const inter = Inter({
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
});

const Footer: React.FC = () => {
  return (
    <footer className={`${inter.className} bg-[#030712] text-gray-400 py-4`}>
      <div className="container mx-auto flex flex-col md:flex-row justify-around items-center px-4">
        <div className="flex space-x-2 ">
          <Shield className="text-[#2563EB]" data-testid="shield-icon" />
          <span className="text-[#FAFAFA] font-semibold text-xl">
            QualiNova
          </span>
        </div>

        <nav className="flex space-x-6 text-sm text-[#9CA3AF]">
          <a href="#" className="hover:text-white">
            Terms
          </a>
          <a href="#" className="hover:text-white">
            Privacy
          </a>
          <a href="#" className="hover:text-white">
            Contact
          </a>
        </nav>

        <p className="text-sm text-[#9CA3AF]">
          &copy; 2025 QualiNova. All rights reserved.
        </p>
      </div>
    </footer>
  );
};

export default Footer;
