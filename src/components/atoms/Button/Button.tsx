"use client";

import React from "react";

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  className?: string;
}

const Button: React.FC<ButtonProps> = ({ children, onClick, className }) => {
  return (
    <button className={`px-3 py-2 rounded-sm hover:opacity-80 ${className}`} onClick={onClick}>
      {children}
    </button>
  );
};

export default Button;
