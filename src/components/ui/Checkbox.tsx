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
            className="w-4 h-4 accent-blue-600  border-gray-700 rounded focus:ring-2 focus:ring-blue-500 cursor-pointer"
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
