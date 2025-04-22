"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import React from "react";

interface NavLinkProps {
  href: string;
  children: React.ReactNode;
}

const NavLink: React.FC<NavLinkProps> = ({ href, children }) => {
  const pathname = usePathname();
  const isActive = pathname === href;

  return (
    <Link href={href} className={`px-2 ${isActive ? "text-white" : "text-[#FAFAFA99]"}`}>
      {children}
    </Link>
  );
};

export default NavLink;
