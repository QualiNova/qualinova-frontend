import React, { ButtonHTMLAttributes, forwardRef } from "react";
import cn from "classnames";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "outline";
  fullWidth?: boolean;
  isLoading?: boolean;
  className?: string;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      children,
      variant = "primary",
      fullWidth = false,
      isLoading = false,
      className,
      disabled,
      ...props
    },
    ref
  ) => {
    const baseStyles =
      "px-4 py-2 rounded-lg font-medium transition-all duration-200 flex items-center justify-center";

    const variants = {
      primary:
        "bg-primary text-white disabled:bg-gray-500 disabled:cursor-not-allowed",
      outline:
        "border border-primary text-primary disabled:opacity-50 disabled:cursor-not-allowed",
    };

    return (
      <button
        ref={ref}
        className={cn(
          baseStyles,
          variants[variant],
          fullWidth && "w-full",
          (disabled || isLoading) && "opacity-50 cursor-not-allowed",
          "cursor-pointer",
          className
        )}
        disabled={disabled || isLoading}
        {...props}
      >
        {isLoading ? (
          <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
        ) : (
          children
        )}
      </button>
    );
  }
);

Button.displayName = "Button";
