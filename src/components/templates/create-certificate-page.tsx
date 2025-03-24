import Navbar from "@/components/organisms/navbar"
import CertificateForm from "@/components/organisms/certificate-form"
import { Button } from "@/components/atoms/Button/Button";

import { ArrowLeft } from "lucide-react"

export default function CreateCertificatePage() {
  return (
    <div className="min-h-screen bg-background">
      <Navbar />
      <main className="container py-8 bg-[#121212] max-w-4xl mx-auto">

        <Button variant="outline" className="mb-6 flex items-center gap-2">
          <ArrowLeft size={16} />
          Back
        </Button>

        <div className="max-w-3xl mx-auto">
          <h1 className="text-2xl font-bold mb-2">Create New Certificate</h1>
          <p className="text-muted-foreground mb-6">
            Fill out the form below to create a new blockchain-verified certificate
          </p>

          <CertificateForm />
        </div>
      </main>
    </div>
  )
}

