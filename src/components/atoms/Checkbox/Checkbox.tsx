import { forwardRef } from "react";

interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: React.ReactNode;
  error?: string;
}

const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(
  ({ label, error, ...props }, ref) => {
    return (
      <div className="flex flex-col space-y-1">
        <div className="flex items-center space-x-2">
          <input
            ref={ref}
            type="checkbox"
            {...props}
            className="w-4 h-4 border border-gray-700 rounded focus:ring-2 focus:ring-blue-500 cursor-pointer appearance-none checked:bg-[#2563EB]  relative 
              checked:after:content-['✓'] checked:after:text-white checked:after:absolute 
              checked:after:top-[50%] checked:after:left-[50%] checked:after:transform 
              checked:after:-translate-x-1/2 checked:after:-translate-y-1/2 checked:after:text-xs"
            aria-describedby={error ? "checkbox-error" : undefined}
          />
          <label className="text-gray-300 text-sm">{label}</label>
        </div>
        {error && (
          <p id="checkbox-error" className="text-red-500 text-xs">
            {error}
          </p>
        )}
      </div>
    );
  }
);

Checkbox.displayName = "Checkbox";

export default Checkbox;
