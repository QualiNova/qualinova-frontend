import { useState } from "react";
import { Eye, EyeOff } from "lucide-react";

interface PasswordInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  error?: string;
}

const PasswordInput: React.FC<PasswordInputProps> = ({
  label,
  error,
  id,
  ...props
}) => {
  const [showPassword, setShowPassword] = useState(false);
  const inputId = id || "password-input";

  return (
    <div className="flex flex-col space-y-1">
      <label htmlFor={inputId} className="text-gray-300 text-sm">
        {label}
      </label>
      <div className="relative">
        <input
          id={inputId} // Ensure input has an ID
          type={showPassword ? "text" : "password"}
          className="w-full p-2 pr-10 border border-[#27272A] rounded-[6px] focus:ring-2 focus:ring-slate-500 text-white"
          {...props}
        />
        <button
          type="button"
          onClick={() => setShowPassword((prev) => !prev)}
          className="absolute right-3 top-2 text-gray-400 hover:text-white"
        >
          {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
        </button>
      </div>
      {error && <p className="text-red-500 text-xs">{error}</p>}
    </div>
  );
};

export default PasswordInput;
