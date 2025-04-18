interface StepIndicatorProps {
    currentStep: number
    totalSteps: number
  }
  
  export default function StepIndicator({ currentStep, totalSteps }: StepIndicatorProps) {
    return (
      <div className="text-sm text-muted-foreground">
        Step {currentStep} of {totalSteps}
      </div>
    )
  }
  
  