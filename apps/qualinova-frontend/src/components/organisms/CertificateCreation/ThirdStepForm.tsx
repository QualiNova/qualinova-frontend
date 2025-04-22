import Image from "next/image";
import warningIcon from "@/components/atoms/icons/warning.svg";
import filePreview from "@/components/atoms/icons/file-preview.svg";
import uploadIcon from "@/components/atoms/icons/upload.svg";
import { Button } from "@/components/atoms/Button/Button";

export default function ThirdStepForm() {
    return (
        <form className="w-full max-w-[832px] min-w-[320px] border border-[#27272A] rounded-lg flex flex-col items-start px-[1px] py-6 shadow-sm">
            <div className="w-full px-7">
                <div className="w-full flex items-center justify-between gap-5">
                    <h3 className="text-[#FAFAFA] font-semibold text-2xl">Certificate Details</h3>
                    <h4 className="text-[#A1A1AA] font-normal text-sm">Step 3 of 3</h4>
                </div>
                <p className="mt-[6px] text-[#A1A1AA] font-normal text-sm">Review and confirm certificate details</p>
            </div>

            <div className="border border-[#27272A] rounded-lg flex items-start text-[#A1A1AA] gap-2 w-full py-4 px-7 mt-7">
                <Image src={warningIcon} alt="warning" width={18} height={18} className="object-cover object-center mt-1" />

                <div className="space-y-1">
                    <h3>Please Review Carefully</h3>
                    <p>Once a certificate is registered on the blockchain, its core details cannot be modified</p>
                </div>
            </div>

            <div className="text-[#FAFAFA] border border-[#27272A] rounded-lg w-full flex flex-col gap-4 mt-7 py-4 px-7">
                <div className="flex w-full">
                    <h4 className="w-[50%]">Certificate Name:</h4>
                    <p className="w-[50%]">ISO 9001 Quality Management</p>
                </div>
                <div className="flex w-full">
                    <h4 className="w-[50%]">Certificate Type</h4>
                    <p className="w-[50%]">Quality Management</p>
                </div>
                <div className="flex w-full">
                    <h4 className="w-[50%]">Recipient</h4>
                    <p className="w-[50%]">Acme Corporation</p>
                </div>
                <div className="flex w-full">
                    <h4 className="w-[50%]">Issue Date</h4>
                    <p className="w-[50%]">Jan 15, 2025</p>
                </div>
                <div className="flex w-full">
                    <h4 className="w-[50%]">Expiry Date</h4>
                    <p className="w-[50%]">Jan 15, 2026</p>
                </div>
                <div className="flex w-full">
                    <h4 className="w-[50%]">Certificate ID</h4>
                    <p className="w-[50%]">CERT-2005-0042 (Auto-generated)</p>
                </div>
            </div>

            <div className="text-[#FAFAFA] border border-[#27272A] rounded-lg w-full flex items-center justify-center mt-7 p-4">
                <div className="size-[95%] flex items-center justify-center gap-3 flex-col border-2 border-dashed border-[#27272A] py-4">
                    <Image src={filePreview} alt="file-preview" width={30} height={30} />
                    <p>Certificate Preview</p>

                    <Button className="border border-[#27272A]">
                        <Image src={uploadIcon} alt="upload" width={20} height={20} className="mr-2" />
                        Preview Certificate
                    </Button>
                </div>
            </div>

            <div className="w-full flex items-center justify-between gap-5 p-6">
                <button type="button" className="bg-[#09090B] border border-[#27272A] text-[#FAFAFA] rounded-md py-[9.5px] px-[17px] text-sm font-medium cursor-pointer transform hover:scale-95 duration-200">
                    Previous Step
                </button>
                <button type="button" className="bg-[#2563EB] text-[#18181B] rounded-md py-[9.5px] px-[17px] text-sm font-medium cursor-pointer transform hover:scale-95 duration-200">
                    Create Certificate
                </button>
            </div>
        </form>
    );
}