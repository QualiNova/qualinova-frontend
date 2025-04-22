import type { InputHTMLAttributes } from "react"
import { cn } from "@/lib/utils"

interface RadioProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string
}

export default function Radio({ className, label, ...props }: RadioProps) {
  return (
    <label className="flex items-center space-x-2 cursor-pointer">
      <input
        type="radio"
        className={cn("h-4 w-4 text-primary border-input focus:ring-primary", className)}
        {...props}
      />
      <span className="text-sm font-medium">{label}</span>
    </label>
  )
}
