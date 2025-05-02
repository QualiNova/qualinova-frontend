"use client"

import Image from 'next/image'
import React, { useState } from 'react'
enum Status {
    Active = "Active",
    Pending = "Pending",
    Expired = "Expired",
}
const certificates = [
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Active,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Active,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Pending,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Expired,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Active,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Active,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Pending,
        actions: "...",
    },
    {
        id: "CERT-2023-001",
        name: "ISO 9001 Compliance",
        recipient: "Acme Corporation",
        issue_date: "14/12/2023",
        expiry_date: "14/12/2024",
        status: Status.Active,
        actions: "...",
    },
]

const page = () => {
    const
        // 
        [total, setTotal] = useState(8),
        [max, setMax] = useState(9);
    return (
        <div className='p-5 space-y-4'>
            {/* Heading */}
            <div className='flex justify-between  items-center w-full'>
                <div className='text-xl font-bold'>
                    Certificates
                </div>
                <div>
                    <button type="button" className='bg-[#3664ef] p-3 rounded-lg space-x-4 flex'>
                        <Image src="/newCert.svg" alt="+" width={20} height={20} />
                        <span className=''>
                            New Certificate
                        </span>
                    </button>
                </div>
            </div>
            <div className='border-2 border-[#20293c] p-4 space-y-5 rounded-lg'>
                {/* Background */}
                <div>
                    {/* certificate management */}
                    <div className=''>Certificate Management</div>
                    <div className='text-[#717c91] text-xs'>View, filter and manage all your blockchain certificates</div>
                </div>
                <div className='flex space-x-3'>
                    {/* search */}
                    <div className='p-2 w-full flex border-2  border-[#20293c] space-x-3 rounded-lg '>
                        {/* input */}
                        <Image src="/search.svg" alt="+" width={20} height={20} className="" />
                        <input type="search" name="" className='bg-inherit lg:w-full focus:outline-none' id="" placeholder='Search' />
                    </div>
                    <div className='flex *:border-2 *:border-[#20293c] *:rounded-lg space-x-3'>
                        {/* filters */}
                        <div className='flex space-x-3 items-center px-2 '>

                            <Image src="/filter.svg" alt="+" width={20} height={20} className="" />
                            <select name="" id="" className='w-40 focus:outline-none bg-inherit'>
                                <option className=' text-black'>All Certificates</option>
                                <option className=' text-black'>option2</option>
                                <option className=' text-black'>option3</option>
                            </select>
                        </div>
                        <div className='flex items-center p-2'>
                            <Image src="/reset.svg" alt="+" width={45} height={45} className="" />
                        </div>
                    </div>

                    {/* <div></div> */}
                </div>
                <div className='flex justify-center w-full items-center'>
                    {/* main */}
                    <div className='border-2 border-[#20293c] flex justify-center rounded-lg p-4 w-full'>
                        <table className='table-auto w-3/4 border-collapse ' style={{ borderRadius: "0.5rem" }}>
                            <thead>
                                <tr className='space-x-3 border-[#20293c] *:py-3 text-left '>
                                    <th className=''>ID</th>
                                    <th className=' w-1/4'>Name</th>
                                    <th className=''>Recipient</th>
                                    <th className=''>Issue Date</th>
                                    <th className=''>Expiry Date</th>
                                    <th className=''>Status</th>
                                    <th className='text-center'>Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    certificates.map((cert, index) => {
                                        return (
                                            <tr className='space-x-3 border-[#20293c] *: *:py-5 border-t' key={index}>
                                                <td className=''>{cert.id}</td>
                                                <td className='flex w-fit '> <Image alt="certLogo" className="mr-3" src="/cert.svg" width={15} height={40} ></Image> {cert.name}</td>
                                                <td className=''>{cert.recipient}</td>
                                                <td className=''>{cert.issue_date}</td>
                                                <td className=''>{cert.expiry_date}</td>
                                                <td className={` *:font-bold *:rounded-xl text-center ${cert.status == Status.Active ? "" : cert.status == Status.Pending ? " " : cert.status == Status.Expired ? "  " : null} `}>
                                                    {cert.status == Status.Active ? <div className={` text-[#256532] bg-[#dffce6]`}>

                                                        {Status.Active}
                                                    </div>
                                                        : cert.status == Status.Pending ? <div className={`text-[#923208] bg-[#dbcdb8]`}>

                                                            {Status.Pending}
                                                        </div>
                                                            : cert.status == Status.Expired ? <div className={`bg-[#256532] text-white`}>

                                                                {Status.Expired}
                                                            </div>
                                                                : null}
                                                </td>
                                                <td className='text-center'>{cert.actions}</td>
                                            </tr>
                                        )
                                    })
                                }
                            </tbody>
                        </table>
                    </div>


                </div>
                <div className='flex justify-between'>
                    {/* previous */}
                    <div className='w-full'>
                        showing {total} out of {max} certificates
                    </div>
                    <div className='flex *:border-2 space-x-3 *:rounded-lg *:p-2 *:border-[#20293c]'>
                        <button type="button">Previous</button>
                        <button type="button">Next</button>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default page