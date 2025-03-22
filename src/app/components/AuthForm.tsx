'use client'

import { AUTH_LABELS, INPUT_TYPES } from "@/constants";
import Image from "next/image";
import Link from "next/link";
import { ChangeEvent, FormEvent, useState } from "react";

type AuthFormProps = {
    type: 'SIGN_IN' | 'SIGN_UP';
    defaultValues: { [key: string]: string };
}

const AuthForm = ({ type, defaultValues }: AuthFormProps) => {
    const [formData, setFormData] = useState(defaultValues);

    const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    };

    const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        console.log(formData);

        // Clearing the form after successful submission for better UX
        setFormData(defaultValues)
    }
    return (
        <form className="flex flex-col gap-4 border border-gray-400 p-5 rounded-md mt-7" onSubmit={handleSubmit}>
            {Object.keys(defaultValues).map((key) => (
                <div key={key} className="flex flex-col gap-1">
                    <div className="flex justify-between w-full">
                        <label htmlFor={key} className="capitalize text-gray-300">{AUTH_LABELS[key]}:</label>

                        {key === 'password' && type === 'SIGN_IN' && (
                            <Link href="#" className="text-sm text-blue-600">Forgot Password?</Link>
                        )}
                    </div>
                    <input
                        id={key}
                        name={key}
                        required
                        type={INPUT_TYPES[key] || 'text'}
                        value={formData[key] || ''}
                        onChange={handleChange}
                        className="text-white p-2 rounded-md border border-gray-500 w-full"
                    />
                </div>
            )
            )}

            <div className="flex items-center gap-2">
                <input type="checkbox" className="relative appearance-none size-[18px] border border-gray-500 rounded-sm cursor-pointer" />
                <label className="text-gray-300">
                    {type === 'SIGN_IN' ? 'Remember Me' : (
                        <span>I agree to the <a className="text-blue-600">terms of service</a> and <a className="text-blue-600">privacy policy</a></span>
                    )}
                </label>
            </div>

            <button className="bg-blue-600 w-full py-2 rounded-md cursor-pointer">
                {type === 'SIGN_IN' ? 'Sign In' : 'Sign Up'}
            </button>

            <div className="flex gap-0.5 items-center">
                <div className="h-[1px] basis-[30%] bg-gray-500" />
                <div className="basis-[45%] text-gray-500 text-sm text-center">OR CONTINUE WITH</div>
                <div className="h-[1px] basis-[30%] bg-gray-500" />
            </div>

            <button type="submit" className="flex items-center justify-center gap-2 w-full py-2 rounded-md border border-gray-600 cursor-pointer">
                <Image src="/icons/github.svg" alt="github" width={20} height={20} className="object-contain" />
                GitHub
            </button>

            <p className="text-center text-sm text-gray-400">
                {type === 'SIGN_UP' ? "Already have an account?" : "Don't have an account?"} {" "}
                <Link href={type === 'SIGN_UP' ? "/sign-in" : '/sign-up'} className="text-blue-600">
                    {type === 'SIGN_UP' ? "Sign In" : "Sign Up"}
                </Link>
            </p>
        </form>
    )
}

export default AuthForm