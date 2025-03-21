"use client";

import Link from 'next/link';
import { X, Menu } from "lucide-react";
import React, { useState } from 'react';
import logo from "../../../../public/logo.svg"
import Image from 'next/image';
import { usePathname } from "next/navigation";
import NavLink from './NavLink';



const Header = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const pathname = usePathname();
  

  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
  };

  return (
    <nav className="bg-black text-[#FAFAFA99] w-full flex justify-center transition-colors">
      {/* Border Wrapper */}
      <div className="flex flex-col w-[80%] p-4">
        <div className="flex items-center  w-full">
          <div className='h-10 w-36'>
            <Image src={logo} className='!relative' alt='logo' fill />
          </div>

          {/* Navigation Links - Desktop */}
          <div className="hidden ml-10 md:flex space-x-6 px-2">
            <NavLink href='/'>Home</NavLink>
            <NavLink href='/dashboard'>Dashboard</NavLink>
            <NavLink href='/create-certificate'>Create Certificate</NavLink>
            <NavLink href='/verify'>Verify</NavLink>
            <NavLink href='/certificate'>Certificates</NavLink>
          </div>

          {/* Buttons - Desktop */}
          <div className="hidden md:flex items-center text-white space-x-4 ml-auto">
            <button>
              Sign In
            </button>
            <button className='bg-[#2563EB] text-black px-3 py-2 rounded-sm hover:opacity-80'>
              Sign Up
            </button>
          </div>

          {/* Mobile Menu Button */}
          <button className="md:hidden p-2 ml-auto" onClick={toggleMenu}>
            {isMenuOpen ? (
              <X className="w-6 h-6" />
            ) : (
              <Menu className="w-6 h-6" />
            )}
          </button>
        </div>

        {/* Mobile Menu */}
        {isMenuOpen && (
          <div className="md:hidden mt-4 flex flex-col space-y-4">
            <NavLink href='/'>Home</NavLink>
            <NavLink href='/dashboard'>Dashboard</NavLink>
            <NavLink href='/create-certificate'>Create Certificate</NavLink>
            <NavLink href='/verify'>Verify</NavLink>
            <NavLink href='/certificate'>Certificates</NavLink>
            <div className="text-white space-y-4 flex flex-col">
                <button>
                Sign In
                </button>
                <button className='bg-[#2563EB] text-black px-3 py-2 rounded-sm hover:opacity-80'>
                Sign Up
                </button>
          </div>
          </div>
        )}
      </div>
    </nav>
  );
};

export default Header;
