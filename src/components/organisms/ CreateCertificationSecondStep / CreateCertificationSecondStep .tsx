"use client";

import { useState } from "react";
import Input from "@/components/atoms/Input/Input";
import { Button } from "@/components/atoms/Button/Button";

export default function CreateCertificationSecondStep() {


    return (
        <form className="w-full max-w-[832px] min-w-[320px] border border-[#27272A] rounded-lg flex flex-col items-start px-[1px] py-6 text-red-500 shadow-sm">
            <div className="w-full px-4 md:px-7">
                <div className="w-full flex items-center justify-between gap-5">
                    <h3 className="text-[#FAFAFA] font-semibold text-lg md:text-2xl">Certificate Details</h3>
                    <h4 className="text-[#A1A1AA] font-normal text-xs md:ext-sm">Step 2 of 3</h4>
                </div>
                <p className="mt-[6px] text-[#A1A1AA] font-normal text-sm">Add recipient and validity details</p>
            </div>

            <div className="w-full flex flex-col gap-4 mt-7 justify-center items-center">
                <div className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                    <Input
                        placeholder="e.g Acme Corporation"
                        type="text"
                        label="Recipient Name"
                    />
                </div>

                <div className="w-full flex flex-col py-[5.5px] gap-[12.5px]">
                    <Input
                        placeholder="e.g contact@acmecorp.com"
                        type="email"
                        label="Recipient Email"
                    />
                </div>

                <div className="w-full max-w-[782px] flex items-center justify-between gap-4">
                    <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">



                        <Input
                            placeholder="e.g contact@acmecorp.com"
                            type="date"
                            label="Issue Date"
                        />
                    </label>

                    <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">



                        <Input
                            placeholder="e.g contact@acmecorp.com"
                            type="date"
                            label="Expiry Date"
                        />
                    </label>
                </div>

                <label className="w-full flex flex-col py-[5.5px] gap-[12.5px]">


                    <Input
                        placeholder="Leave blank to autogenerate"
                        type="text"
                        label="Certificate ID (Optional)"
                    />
                    <p className="text-[#A1A1AA] font-normal text-xs">A unique identifier will be automatically generated if left blank</p>
                </label>
            </div>

            <div className="w-full flex items-center justify-between gap-5 p-6">

                <Button
                    type="button"


                    className="bg-[#09090B]  border border-[#27272A]  rounded-md py-[9.5px] px-[17px]  cursor-pointer transform hover:scale-95 duration-200"
                >
                    <p className=" text-sm font-medium   text-[#FAFAFA]">

                        Previous Step
                    </p>
                </Button>






                <Button
                    type="button"


                    className="bg-[#FAFAFA] w-[131px] rounded-md py-[9.5px] px-[17px] text-sm font-medium cursor-pointer transform hover:scale-95 duration-200"
                >
                    <p className=" text-sm font-medium   text-[#18181B]">
                        Next Step

                    </p>
                </Button>
            </div>
        </form>
    );
}
