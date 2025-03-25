
import Link from "next/link"
export default function Logo() {
  return (
    <Link href="/" className="flex items-center gap-2">
      <div className="text-primary">
        <svg width="18" height="22" viewBox="0 0 18 22" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            d="M17 12C17 17 13.5 19.5 9.34 20.95C9.12216 21.0238 8.88554 21.0202 8.67 20.94C4.5 19.5 1 17 1 12V4.99996C1 4.73474 1.10536 4.48039 1.29289 4.29285C1.48043 4.10532 1.73478 3.99996 2 3.99996C4 3.99996 6.5 2.79996 8.24 1.27996C8.45185 1.09896 8.72135 0.999512 9 0.999512C9.27865 0.999512 9.54815 1.09896 9.76 1.27996C11.51 2.80996 14 3.99996 16 3.99996C16.2652 3.99996 16.5196 4.10532 16.7071 4.29285C16.8946 4.48039 17 4.73474 17 4.99996V12Z"
            stroke="#2563EB"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      </div>
      <span className="font-bold text-lg">QualiNova</span>
    </Link>
  )
}

