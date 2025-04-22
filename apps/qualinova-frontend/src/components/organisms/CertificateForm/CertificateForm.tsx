"use client"
import type React from "react"
import { useState } from "react"
import { Button } from "@/components/atoms/Button/Button";
import Input from "@/components/atoms/Input/Input";
import Textarea from "@/components/atoms/textarea/textarea"
import Select from "@/components/atoms/select/select";
import Radio from "@/components/atoms/radio/radio";
import FormField from "@/components/molecules/FormField/FormField"
import StepIndicator from "@/components/molecules/StepIndicator/StepIndicator"
import { ArrowRight } from "lucide-react"
interface FormData {
  name: string
  type: string
  description: string
  template: string
}

interface FormErrors {
  name?: string
  type?: string
  description?: string
}

const submitCertificateStep1 = async ( ): Promise<{ success: boolean; message: string }> => {
  await new Promise((resolve) => setTimeout(resolve, 1500))

  return {
    success: true,
    message: "Step 1 data saved successfully",
  }
}

export default function CertificateForm() {
  const [formData, setFormData] = useState<FormData>({
    name: "",
    type: "",
    description: "",
    template: "standard",
  })

  const [errors, setErrors] = useState<FormErrors>({})
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [submitStatus, setSubmitStatus] = useState<{
    success?: boolean
    message?: string
  }>({})

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))

    if (errors[name as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }))
    }
    if (submitStatus.message) {
      setSubmitStatus({})
    }
  }

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {}

    if (!formData.name.trim()) {
      newErrors.name = "Certificate name is required"
    } else if (formData.name.trim().length < 3) {
      newErrors.name = "Certificate name must be at least 3 characters"
    }

    if (!formData.type) {
      newErrors.type = "Please select a certificate type"
    }

    if (!formData.description.trim()) {
      newErrors.description = "Description is required"
    } else if (formData.description.trim().length < 10) {
      newErrors.description = "Description must be at least 10 characters"
    }

    setErrors(newErrors)
    return Object.keys(newErrors).length === 0
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validateForm()) {
      setSubmitStatus({
        success: false,
        message: "Please fix the errors before proceeding",
      })
      return
    }

    setIsSubmitting(true)
    setSubmitStatus({})

    try {
      const response = await submitCertificateStep1()
      if (response.success) {
        localStorage.setItem("certificateFormStep1", JSON.stringify(formData))
        setSubmitStatus({
          success: true,
          message: response.message,
        })

        setTimeout(() => {
          alert("In a real application, you would now be redirected to Step 2")
        }, 1000)
      } else {
        throw new Error(response.message || "Failed to submit form")
      }
    } catch (error) {
      console.error("Error submitting form:", error)
      setSubmitStatus({
        success: false,
        message: error instanceof Error ? error.message : "An unexpected error occurred",
      })
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="bg-secondary rounded-md p-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Certificate Details</h2>
        <StepIndicator currentStep={1} totalSteps={3} />
      </div>
      <p className="text-sm text-muted-foreground mb-6">Enter the basic certificate information</p>

      {submitStatus.message && (
        <div
          className={`p-3 mb-4 rounded-md ${submitStatus.success ? "bg-green-900/20 text-green-400" : "bg-red-900/20 text-red-400"}`}
        >
          {submitStatus.message}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-6">
        <FormField label="Certificate Name" htmlFor="name">
          <Input
          label="Name"
            id="name"
            name="name"
            placeholder="e.g. ISO 9001 Quality Management"
            value={formData.name}
            onChange={handleChange}
            error={errors.name}
            disabled={isSubmitting}
            aria-invalid={!!errors.name}
            aria-describedby={errors.name ? "name-error" : undefined}
          />
          {errors.name && (
            <div id="name-error" className="sr-only">
              {errors.name}
            </div>
          )}
        </FormField>

        <FormField label="Certificate Type" htmlFor="type">
          <Select
            id="type"
            name="type"
            value={formData.type}
            onChange={handleChange}
            error={errors.type}
            disabled={isSubmitting}
            aria-invalid={!!errors.type}
            aria-describedby={errors.type ? "type-error" : undefined}
          >
            <option value="" disabled>
              Select certificate type
            </option>
            <option value="quality">Quality</option>
            <option value="compliance">Compliance</option>
            <option value="achievement">Achievement</option>
          </Select>
          {errors.type && (
            <div id="type-error" className="sr-only">
              {errors.type}
            </div>
          )}
        </FormField>

        <FormField label="Description" htmlFor="description">
          <Textarea
            id="description"
            name="description"
            placeholder="Describe what this certificate represents..."
            value={formData.description}
            onChange={handleChange}
            rows={4}
            error={errors.description}
            disabled={isSubmitting}
            aria-invalid={!!errors.description}
            aria-describedby={errors.description ? "description-error" : undefined}
          />
          {errors.description && (
            <div id="description-error" className="sr-only">
              {errors.description}
            </div>
          )}
        </FormField>

        <div className="space-y-2">
          <label className="block text-sm font-medium">Certificate Template</label>
          <div className="space-y-2">
            <Radio
              name="template"
              value="standard"
              label="Standard Template"
              checked={formData.template === "standard"}
              onChange={handleChange}
              disabled={isSubmitting}
            />
            <Radio
              name="template"
              value="premium"
              label="Premium Template"
              checked={formData.template === "premium"}
              onChange={handleChange}
              disabled={isSubmitting}
            />
            <Radio
              name="template"
              value="custom"
              label="Custom Template"
              checked={formData.template === "custom"}
              onChange={handleChange}
              disabled={isSubmitting}
            />
          </div>
        </div>

<div className="flex justify-end">
  <Button
    type="submit"
    className="flex items-center gap-2 bg-white !text-black hover:bg-gray-200"
    disabled={isSubmitting}
  >
    {isSubmitting ? (
      <>
        <span className="animate-pulse">Processing...</span>
        <span className="animate-spin h-4 w-4 border-2 border-current border-t-transparent rounded-full"></span>
      </>
    ) : (
      <>
        <span className="!text-black">Next Step</span>
        <ArrowRight size={16} className="!text-black" />
      </>
    )}
  </Button>
</div>



      </form>
    </div>
  )
}


