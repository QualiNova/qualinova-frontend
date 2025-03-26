"use client";

import React from "react";
import Image from "next/image";
import NavMenu from "../../molecules/NavMenu/NavMenu";

const Header = () => {
  return (
    <nav className="bg-black text-[#FAFAFA99] w-full flex justify-center transition-colors">
      <div className="flex flex-col w-full md:w-[80%] p-4">
        <div className="flex flex-col md:flex-row items-center w-full">
          <div className="h-10 w-36 mr-auto mr:0">
            <Image src="/logo.svg" className="!relative" alt="logo" fill />
          </div>
          <NavMenu />
        </div>
      </div>
    </nav>
  );
};

export default Header;
