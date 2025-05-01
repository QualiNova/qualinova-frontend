"use client";

import React from "react";
import Image from "next/image";
import NavMenu from "../../molecules/NavMenu/NavMenu";

const Header = () => {
  return (
    <nav className="bg-[#030817] text-[#FAFAFA99] flex justify-center border-b-2 border-[#1c2537] transition-colors">
      <div className="flex flex-col w-full max-w-[75%] py-4">
        <div className="flex flex-col md:flex-row items-center w-full">
          <div className="h-10 w-36">
            <Image src="/logo.svg" className="!relative" alt="logo" fill />
          </div>
          <NavMenu />
        </div>
      </div>
    </nav>
  );
};

export default Header;
