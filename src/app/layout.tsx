import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import Footer from "@/components/organisms/Footer/Footer";
import Header from "@/components/organisms/Header/Header";
const inter = Inter({ subsets: ["latin"] })
export const metadata: Metadata = {
  title: "QualitNova - Create Certificate",
  description: "Create blockchain-verified certificates",
  viewport: "width=device-width, initial-scale=1, maximum-scale=1",
}
export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="flex flex-col">
        <Header />
        <main className={`${inter.className} flex-grow`}>{children}</main>
        <Footer />
      </body>
    </html>
  )
}

