"use client";

import { useState } from "react";

export default function SecondStepForm() {
    const [startDate, setStartDate] = useState("");
    const [endDate, setEndDate] = useState("");



    return (
        <form className="w-full max-w-[832px] min-w-[320px] border border-[#27272A] rounded-lg flex flex-col items-start px-[1px] py-6 text-red-500 shadow-sm">
            <div className="w-full px-7">
                <div className="w-full flex items-center justify-between gap-5">
                    <h3 className="text-[#FAFAFA] font-semibold text-2xl">Certificate Details</h3>
                    <h4 className="text-[#A1A1AA] font-normal text-sm">Step 2 of 3</h4>
                </div>
                <p className="mt-[6px] text-[#A1A1AA] font-normal text-sm">Add recipient and validity details</p>
            </div>

            <div className="w-full flex flex-col gap-4 mt-7 justify-center items-center">
                <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                    <span className="text-[#FAFAFA] font-medium text-sm">Recipient Name</span>
                    <input type="text" placeholder="e.g Acme Corporation" className="border border-[#27272A] outline-none bg-[#09090B] rounded-md h-10 text-sm font-normal text-[#A1A1AA] px-[13px]" />
                </label>

                <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                    <span className="text-[#FAFAFA] font-medium text-sm">Recipient Email</span>
                    <input type="email" placeholder="e.g Acme Contact@acmecorp.com" className="border border-[#27272A] outline-none bg-[#09090B] rounded-md h-10 text-sm font-normal text-[#A1A1AA] px-[13px]" />
                </label>

                <div className="w-full max-w-[782px] flex items-center justify-between gap-4">
                    <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                        <span className="text-[#FAFAFA] font-medium text-sm">Issue Date</span>
                        <input
                            type="date"
                            value={startDate}
                            min="2024-01-01"
                            max="2025-12-31"
                            onChange={(e) => setStartDate(e.target.value)}
                            className="border border-[#27272A] outline-none bg-[#09090B] rounded-md h-10 text-sm font-normal text-[#A1A1AA] px-[13px]"
                        />
                    </label>

                    <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                        <span className="text-[#FAFAFA] font-medium text-sm">Expiry Date</span>
                        <input
                            type="date"
                            value={endDate}
                            min="2024-01-01"
                            max="2025-12-31"
                            onChange={(e) => setEndDate(e.target.value)}
                            className="border border-[#27272A] outline-none bg-[#09090B] rounded-md h-10 text-sm font-normal text-[#A1A1AA] px-[13px]"
                        />
                    </label>
                </div>

                <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                    <span className="text-[#FAFAFA] font-medium text-sm">Certificate ID (Optional)</span>
                    <input type="text" placeholder="Leave blank to autogenerate" className="border border-[#27272A] outline-none bg-[#09090B] rounded-md h-10 text-sm font-normal text-[#A1A1AA] px-[13px]" />
                    <p className="text-[#A1A1AA] font-normal text-xs">Add recipient and validity details</p>
                </label>
            </div>

            <div className="w-full flex items-center justify-between gap-5 p-6">
                <button type="button" className="bg-[#09090B] border border-[#27272A] text-[#FAFAFA] rounded-md py-[9.5px] px-[17px] text-sm font-medium cursor-pointer transform hover:scale-95 duration-200">
                    Previous Step
                </button>
                <button type="button" className="bg-[#FAFAFA] text-[#18181B] rounded-md py-[9.5px] px-[17px] text-sm font-medium cursor-pointer transform hover:scale-95 duration-200">
                    Next Step
                </button>
            </div>
        </form>
    );
}
