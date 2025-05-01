
"use client"

import Image from 'next/image'
import React, { useState } from 'react'


const page = () => {
    const
        // 
        [currentPanel, setCurrentPanel] = useState("verify");

    return (
        <div className='flex *:mt-14 flex-col items-center space-y-6 justify-center'>
            <div className='w-7/12  flex items-center flex-col justify-center'>
                <div style={{ width: "75%" }} className='flex  space-y-4 flex-col items-center'>
                    {/* Heading 1*/}
                    <div className='text-4xl font-bold'>Certificate Verification</div>
                    <div className='text-gray-400 text-center text-xl'>
                        Verify the authenticity of any QualiNova certificate using its unique ID or by scanning the
                        QR code
                    </div>
                </div>

                <div className='  w-full' >
                    {/* Tablist */}
                    <div className='*:text-center w-full *:p-2 flex justify-center *:rounded-lg rounded-lg *:w-full p-1 my-10' style={{ backgroundColor: "#20293b" }} >
                        <button type="button" className={currentPanel == "verify" ? " bg-[#030817] " : ""} onClick={() => { setCurrentPanel("verify") }}>Verify by ID</button>
                        <button type="button" className={currentPanel == "QR" ? " bg-[#030817] " : ""} onClick={() => { setCurrentPanel("QR") }}>Verify by QR Code</button>

                    </div>
                    {/* Tab panel */}
                    {
                        currentPanel == "verify" ?
                            <div className='space-y-6 rounded-lg p-6 border-2' style={{ borderColor: "#1c2537" }}>
                                <div>
                                    <div className='text-3xl font-bold'>Enter Certificate ID</div>
                                    <div className='text-gray text-base text-gray-400'>
                                        Enter the unique certificate ID to verify its authenticity on the blockchain
                                    </div>
                                </div>
                                {/* input */}
                                <div className='flex justify-center *:p-3 space-x-3 *:rounded-lg'>
                                    <input type="text" name="" id="" style={{ borderColor: "#1c2537" }} className='w-2/5 border-2 bg-inherit' placeholder='e.g. CERT-2023-001' />
                                    <div className='flex' style={{ backgroundColor: "#1e3580" }}>
                                        <Image alt="search" src="/search.svg" width={25} height={25} />
                                        <button type="button" className='mx-4'>Verify</button>
                                    </div>
                                </div>
                            </div>
                            // QR
                            : currentPanel == "QR" ? <div className=' space-y-10 rounded-lg p-6 border-2 border-[#1c2537]'>
                                {/* text */}
                                <div>
                                    <div className='text-3xl font-bold'>Scan QR Code</div>
                                    <div className='text-gray text-base text-gray-400'>
                                        Scan the QR code on the certificate to verify its authenticity
                                    </div>
                                </div>
                                {/* qr */}
                                <div className='flex p-10 rounded-lg space-y-5 flex-col items-center justify-center'>
                                    <div className='flex p-10 rounded-lg flex-col items-center border-dashed border-2 border-[#1c2537]'><Image alt="qr code" src="QR.svg" width={90} height={90} ></Image>
                                        <div className='flex flex-col space-y-4 items-center'>
                                            <div className='text-2xl '>QR Code Scanner</div>
                                            <div className='text-gray text-center text-gray-400'>
                                                Position the QR code within the scanner area. The verification will
                                                <br></br> start automatically.
                                            </div>
                                        </div>
                                        {/* button */}

                                    </div>
                                    <div className='flex bg-[#3962ea] p-3 space-x-6 rounded-lg justify-center'>
                                        <Image alt="qr code" src="QR.svg" width={20} height={20} ></Image>
                                        <button type="button">Start Scanner</button>
                                    </div>
                                </div>
                            </div> : null
                    }
                </div>
            </div>

        </div>
    )
}

export default page