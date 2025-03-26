"use client"

import { useState } from "react"
import Logo from "@/components/atoms/icons/logo"
import NavItem from "@/components/molecules/nav-item"
import { Button } from "@/components/atoms/Button/Button";
import { Menu, X } from "lucide-react"

export default function Navbar() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen)
  }

  return (
    <header className="border-b border-border">
      <div className="container px-4 sm:px-6 mx-auto">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center">
            <Logo />
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center space-x-1">
            <NavItem href="/" label="Home" />
            <NavItem href="/dashboard" label="Dashboard" />
            <NavItem href="/create-certificate" label="Create Certificate" active />
            <NavItem href="/verify" label="Verify" />
            <NavItem href="/certificates" label="Certificates" />
          </nav>

          {/* Desktop Auth Buttons */}
          <div className="hidden md:flex items-center gap-2">
            <Button variant="outline" className="sm">
              Sign In
            </Button>
            <Button className="bg-[#2563EB] sm:flex-1 text-center">
              Sign Up
            </Button>
          </div>

          {/* Mobile Menu Button */}
          <button
            type="button"
            className="md:hidden inline-flex items-center justify-center p-2 rounded-md text-foreground"
            onClick={toggleMobileMenu}
            aria-expanded={mobileMenuOpen}
            aria-controls="mobile-menu"
          >
            <span className="sr-only">{mobileMenuOpen ? "Close menu" : "Open menu"}</span>
            {mobileMenuOpen ? (
              <X className="h-6 w-6" aria-hidden="true" />
            ) : (
              <Menu className="h-6 w-6" aria-hidden="true" />
            )}
          </button>
        </div>
      </div>

      {/* Mobile Menu */}
      {mobileMenuOpen && (
        <div className="md:hidden" id="mobile-menu">
          <div className="space-y-1 px-4 py-3 border-t border-border">
            <NavItem href="/" label="Home" />
            <NavItem href="/dashboard" label="Dashboard" />
            <NavItem href="/create-certificate" label="Create Certificate" active />
            <NavItem href="/verify" label="Verify" />
            <NavItem href="/certificates" label="Certificates" />
          </div>
          <div className="px-4 py-3 border-t border-border flex gap-2">
            <Button variant="outline" className="sm flex-1 text-center">
              Sign In
            </Button>

            <Button className="bg-[#2563EB] sm:flex-1 text-center">
                Sign Up
            </Button>
          </div>
        </div>
      )}
    </header>
  )
}

