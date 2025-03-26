"use client";

import { Button } from "@/components/atoms/Button/Button";
import Checkbox from "@/components/atoms/Checkbox/Checkbox";
import githubIcon from "@/components/atoms/icons/githubicon.svg";
import Input from "@/components/atoms/Input/Input";
import { loginFormSchema } from "@/schemas/loginFormSchema";
import { zodResolver } from "@hookform/resolvers/zod";
import Image from "next/image";
import Link from "next/link";
import { useForm } from "react-hook-form";
import { z } from "zod";
import PasswordInput from "../../atoms/Input/PasswordInput";

type RegisterFormData = z.infer<typeof loginFormSchema>;

const LoginForm = () => {
    const {
        register,
        handleSubmit,
        formState: { errors, isValid },
    } = useForm<RegisterFormData>({
        resolver: zodResolver(loginFormSchema),
        mode: "onChange",
    });

    const onSubmit = (data: RegisterFormData) => {
        console.log("Form Submitted:", data);
    };

    return (
        <div className="h-screen flex justify-center items-center">
            <div className="space-y-4 py-6 md:py-10 px-4 xl:py-0 xl:px-0">
                <div className="text-center flex flex-col gap-2">
                    <h2 className="text-xl md:text-2xl leading-8 font-semibold text-[#FAFAFA]">
                        Welcome back
                    </h2>
                    <p className="text-sm text-[#A1A1AA] leading-5">
                        Enter your credentials to sign in to your account
                    </p>
                </div>
                <form
                    onSubmit={handleSubmit(onSubmit)}
                    className="w-full max-w-[420px] mx-auto space-y-4 bg-[#09090B] h-fit p-3 md:p-6
           rounded-lg border border-[#27272A] shadow-[0px_1px_2px_#0000000D]"
                >
                    <Input
                        label="Email"
                        placeholder="name@example.com"
                        type="email"
                        {...register("email")}
                        error={errors.email?.message}
                    />

                    <div className="space-y-2">
                        <PasswordInput
                            label="Password"
                            {...register("password")}
                            error={errors.password?.message}
                        />
                        <p className="text-[#A1A1AA] text-xs leading-4">
                            Password must be at least 8 characters long
                        </p>
                    </div>

                    <Checkbox
                        label={
                            <div className="text-sm">
                                Remember me
                            </div>
                        }
                        {...register("rememberMe")}
                        error={errors.rememberMe?.message}
                    />

                    <Button
                        type="submit"
                        fullWidth
                        disabled={!isValid}
                        className="rounded-[6px]  bg-[#2563EB]/50 hover:bg-[#2563EB] text-[#18181B]"
                    >
                        <p className=" text-sm leading-5  text-[#18181B] font-medium">
                            {" "}
                            Sign In
                        </p>
                    </Button>

                    <div className="flex justify-between items-center">
                        <div className="h-[1px] w-[25%] bg-[#27272A]"></div>
                        <p className="text-[#A1A1AA] w-[50%] text-xs text-center">OR CONTINUE WITH</p>
                        <div className="h-[1px] w-[25%] bg-[#27272A]"></div>
                    </div>

                    <Button
                        type="button"
                        className="flex border border-[#27272A] w-full hover:bg-[#27272A] "
                    >
                        <Image
                            src={githubIcon}
                            alt=""
                            width={20}
                            height={20}
                            className="mr-2"
                        />
                        GitHub
                    </Button>

                    <div className="flex justify-center text-sm items-center">
                        <p className="text-[#A1A1AA]">
                            Don&apos;t have an account?{" "}
                            <span>
                                <Link
                                    className="text-sm hover:underline text-[#2563EB]"
                                    href="/register"
                                >
                                    Sign up
                                </Link>
                            </span>
                        </p>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default LoginForm;