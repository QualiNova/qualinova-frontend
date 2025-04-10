import Link from "next/link"
import { cn } from "@/lib/utils"

interface NavItemProps {
  href: string
  label: string
  active?: boolean
}

export default function NavItem({ href, label, active }: NavItemProps) {
  return (
    <Link
      href={href}
      className={cn(
        "px-3 py-2 text-sm font-medium transition-colors hover:text-primary",
        active ? "text-primary" : "text-foreground",
      )}
    >
      {label}
    </Link>
  )
}

