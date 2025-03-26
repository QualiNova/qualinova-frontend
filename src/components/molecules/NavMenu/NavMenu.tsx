"use client";

import React, { useState } from "react";
import { X, Menu } from "lucide-react";
import NavLink from "@/components/atoms/NavLink/NavLink";
import Button from "@/components/atoms/Button/Button";

const NavMenu = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const toggleMenu = () => setIsMenuOpen(!isMenuOpen);

  return (
    <>
     {/* Desktop Navigation */}
      <div className="hidden ml-10 md:flex space-x-6 px-2">
        <NavLink href="/">Home</NavLink>
        <NavLink href="/dashboard">Dashboard</NavLink>
        <NavLink href="/create-certificate">Create Certificate</NavLink>
        <NavLink href="/verify">Verify</NavLink>
        <NavLink href="/certificate">Certificates</NavLink>
      </div>

      {/* Desktop Buttons */}
      <div className="hidden md:flex items-center text-white space-x-4 ml-auto">
        <Button>Sign In</Button>
        <Button className="bg-[#2563EB] text-black">Sign Up</Button>
      </div>

      {/* Mobile Menu Button */}
      <button className="absolute md:hidden p-2 right-5" onClick={toggleMenu}>
        {isMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
      </button>


      {/* Mobile Menu */}
      {isMenuOpen && (
            <div className="md:hidden mt-4 mr-auto ml-10 flex flex-col space-y-4">
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

    </>
  );
};

export default NavMenu;
